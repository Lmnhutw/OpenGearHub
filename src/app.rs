use std::{cell::RefCell, rc::Rc, time::Instant};

use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel};

use crate::{
    AppWindow,
    devices::{
        discovery, registry,
        types::{DeviceGroup, GroupId},
        worker::Worker,
    },
};

#[derive(Clone, Default)]
#[allow(clippy::struct_excessive_bools)] // Independent scan, snapshot, rendering and lifecycle flags.
struct InventoryState {
    groups: Vec<DeviceGroup>,
    selected: Option<GroupId>,
    refreshing: bool,
    has_snapshot: bool,
    error: String,
    closed: bool,
    rendering: bool,
}

impl InventoryState {
    fn begin_refresh(&mut self) -> bool {
        if self.closed || self.refreshing {
            return false;
        }
        self.refreshing = true;
        self.error.clear();
        true
    }

    fn finish(&mut self, result: Result<Vec<DeviceGroup>, String>) {
        if self.closed {
            return;
        }
        self.refreshing = false;
        match result {
            Ok(groups) => {
                if !groups
                    .iter()
                    .any(|group| Some(&group.id) == self.selected.as_ref())
                {
                    self.selected = None;
                }
                self.groups = groups;
                self.has_snapshot = true;
                self.error.clear();
            }
            Err(error) => self.error = error,
        }
    }

    fn select(&mut self, index: i32) {
        self.selected = usize::try_from(index)
            .ok()
            .and_then(|i| self.groups.get(i))
            .map(|group| group.id.clone());
    }

    fn status_text(&self) -> String {
        if self.refreshing {
            return "Đang quét HID ngoài UI thread…".into();
        }
        if !self.error.is_empty() {
            return if self.has_snapshot {
                "Quét thất bại — dữ liệu trước chưa được cập nhật.".into()
            } else {
                "Chưa tạo được inventory. Hãy thử làm mới.".into()
            };
        }
        let count: usize = self.groups.iter().map(|group| group.interfaces.len()).sum();
        format!(
            "Enumerate được {count} interface / {} nhóm. Một số interface không truy cập được có thể bị bỏ qua.",
            self.groups.len()
        )
    }
}

fn group_label(group: &DeviceGroup) -> String {
    let name = group
        .interfaces
        .iter()
        .find_map(|endpoint| endpoint.product.as_deref())
        .unwrap_or("HID chưa biết tên");
    let kind = match group.id {
        GroupId::Container(_) => "Nhóm Windows",
        GroupId::Interface(_) => "Interface chưa ghép",
    };
    format!("{name} — {kind} ({} interface)", group.interfaces.len())
}

fn details(group: &DeviceGroup) -> String {
    use std::fmt::Write;
    let support = match registry::resolve(group) {
        registry::MatchResult::Unsupported => "Chưa hỗ trợ điều khiển — chỉ nhận diện",
        registry::MatchResult::Matched(_) => "Nhận diện model — chưa xác minh điều khiển",
        registry::MatchResult::Ambiguous => "Nhận diện mơ hồ — không điều khiển",
    };
    let mut text = format!(
        "{}\n{support}\n\nNhóm Windows không chứng minh một thiết bị ngoại vi độc lập; receiver có thể chứa nhiều thiết bị logic.\n",
        group_label(group)
    );
    for (index, endpoint) in group.interfaces.iter().enumerate() {
        let interface = endpoint
            .interface_number
            .map_or_else(|| "Chưa biết".into(), |number| number.to_string());
        let _ = write!(
            text,
            "\nInterface {}\nTên: {}\nHãng: {}\nVID:PID: {:04X}:{:04X}\nUsage page / usage: {:04X} / {:04X}\nInterface number: {}\nBus: {}\nRelease (bcdDevice): {:04X}\n",
            index + 1,
            endpoint.product.as_deref().unwrap_or("Chưa biết"),
            endpoint.manufacturer.as_deref().unwrap_or("Chưa biết"),
            endpoint.vendor_id,
            endpoint.product_id,
            endpoint.usage_page,
            endpoint.usage,
            interface,
            endpoint.bus.label(),
            endpoint.release_number
        );
    }
    text
}

fn render(ui: &AppWindow, shared_state: &RefCell<InventoryState>) {
    let state = shared_state.borrow().clone();
    shared_state.borrow_mut().rendering = true;
    let rows: Vec<_> = state
        .groups
        .iter()
        .map(|group| StandardListViewItem::from(slint::SharedString::from(group_label(group))))
        .collect();
    let selected_index = state
        .groups
        .iter()
        .position(|group| Some(&group.id) == state.selected.as_ref());
    let detail = selected_index.map_or_else(|| "Chọn một hàng để xem các interface và metadata.\n\nChưa có model được xác minh điều khiển. Raw path và serial không được hiển thị.".into(), |index| details(&state.groups[index]));
    ui.set_devices(ModelRc::new(VecModel::from(rows)));
    ui.set_selected_index(
        selected_index
            .and_then(|index| i32::try_from(index).ok())
            .unwrap_or(-1),
    );
    ui.set_detail_text(detail.into());
    ui.set_refreshing(state.refreshing);
    ui.set_status_text(state.status_text().into());
    ui.set_error_text(state.error.clone().into());
    shared_state.borrow_mut().rendering = false;
}

fn close(state: &RefCell<InventoryState>, worker: &RefCell<Option<Worker>>) {
    state.borrow_mut().closed = true;
    if let Some(worker) = worker.borrow_mut().as_mut() {
        worker.shutdown();
    }
}

fn request_refresh(state: &mut InventoryState, worker: Option<&mut Worker>) {
    if !state.begin_refresh() {
        return;
    }
    let result = worker
        .ok_or_else(|| "Worker không khả dụng. Hãy mở lại ứng dụng.".into())
        .and_then(Worker::refresh);
    if let Err(error) = result {
        state.finish(Err(error));
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let ui = AppWindow::new()?;
    let state = Rc::new(RefCell::new(InventoryState::default()));
    let weak = ui.as_weak();
    let worker = Worker::spawn(discovery::scan, move || {
        let _ = weak.upgrade_in_event_loop(|ui| ui.invoke_results_ready());
    });
    let worker = Rc::new(RefCell::new(worker.ok()));
    if worker.borrow().is_none() {
        state.borrow_mut().finish(Err(
            "Không tạo được worker. Hãy đóng và mở lại ứng dụng.".into()
        ));
    }

    let callback_state = Rc::clone(&state);
    let callback_worker = Rc::clone(&worker);
    let weak = ui.as_weak();
    ui.on_refresh(move || {
        request_refresh(
            &mut callback_state.borrow_mut(),
            callback_worker.borrow_mut().as_mut(),
        );
        if let Some(ui) = weak.upgrade() {
            render(&ui, &callback_state);
        }
    });

    let callback_state = Rc::clone(&state);
    let callback_worker = Rc::clone(&worker);
    let weak = ui.as_weak();
    ui.on_results_ready(move || {
        if callback_state.borrow().closed {
            return;
        }
        let result = callback_worker
            .borrow_mut()
            .as_mut()
            .ok_or_else(|| "Worker không khả dụng.".into())
            .and_then(Worker::try_result);
        match result {
            Ok(Some(result)) => {
                if let Ok(groups) = &result.inventory {
                    let count: usize = groups.iter().map(|group| group.interfaces.len()).sum();
                    eprintln!(
                        "inventory: {} ms; {count} interfaces; {} groups",
                        result.elapsed.as_millis(),
                        groups.len()
                    );
                }
                callback_state.borrow_mut().finish(result.inventory);
            }
            Ok(None) => return,
            Err(error) => callback_state.borrow_mut().finish(Err(error)),
        }
        if let Some(ui) = weak.upgrade() {
            render(&ui, &callback_state);
        }
    });

    let callback_state = Rc::clone(&state);
    let weak = ui.as_weak();
    ui.on_selection_changed(move |index| {
        if callback_state.borrow().rendering || callback_state.borrow().closed {
            return;
        }
        let detail = {
            let mut state = callback_state.borrow_mut();
            state.select(index);
            usize::try_from(index)
                .ok()
                .and_then(|i| state.groups.get(i))
                .map(details)
                .unwrap_or_default()
        };
        if let Some(ui) = weak.upgrade() {
            ui.set_detail_text(detail.into());
        }
    });

    let closing_state = Rc::clone(&state);
    let closing_worker = Rc::clone(&worker);
    ui.window().on_close_requested(move || {
        close(&closing_state, &closing_worker);
        slint::CloseRequestResponse::HideWindow
    });
    render(&ui, &state);
    ui.show()?;
    eprintln!(
        "window shown: {} ms from main",
        started.elapsed().as_millis()
    );
    ui.invoke_refresh();
    let result = slint::run_event_loop();
    close(&state, &worker);
    result?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(path: &str) -> DeviceGroup {
        DeviceGroup {
            id: GroupId::Interface(path.into()),
            interfaces: vec![],
        }
    }

    #[test]
    fn failed_refresh_preserves_stale_inventory_and_selection() {
        let mut state = InventoryState::default();
        state.finish(Ok(vec![group("synthetic-a")]));
        let original = state.groups.clone();
        state.select(0);
        assert!(state.begin_refresh());
        assert!(!state.begin_refresh());
        state.finish(Err("synthetic error".into()));
        assert_eq!(state.groups, original);
        assert_eq!(
            state.selected,
            Some(GroupId::Interface("synthetic-a".into()))
        );
        assert_eq!(state.error, "synthetic error");
        assert!(state.has_snapshot);
        assert!(state.status_text().contains("chưa được cập nhật"));
        assert!(!state.refreshing);
    }

    #[test]
    fn successful_refresh_preserves_identity_and_clears_removed_selection() {
        let mut state = InventoryState::default();
        state.finish(Ok(vec![group("a"), group("b")]));
        state.select(1);
        state.finish(Ok(vec![group("b")]));
        assert_eq!(state.selected, Some(GroupId::Interface("b".into())));
        state.finish(Ok(vec![group("a")]));
        assert!(state.selected.is_none());
        state.select(-1);
        assert!(state.selected.is_none());
    }

    #[test]
    fn closed_state_rejects_refresh_and_late_result() {
        let mut state = InventoryState {
            closed: true,
            ..InventoryState::default()
        };
        assert!(!state.begin_refresh());
        state.finish(Ok(vec![group("late")]));
        assert!(state.groups.is_empty());
    }

    #[test]
    fn successful_refresh_clears_previous_error() {
        let mut state = InventoryState::default();
        state.finish(Err("synthetic failure".into()));
        assert!(state.begin_refresh());
        state.finish(Ok(vec![group("recovered")]));
        assert_eq!(state.groups, vec![group("recovered")]);
        assert!(state.error.is_empty());
        assert!(!state.refreshing);
    }

    #[test]
    fn unavailable_worker_ends_loading_with_visible_error() {
        let mut state = InventoryState::default();
        request_refresh(&mut state, None);
        assert!(!state.refreshing);
        assert!(!state.has_snapshot);
        assert_eq!(
            state.status_text(),
            "Chưa tạo được inventory. Hãy thử làm mới."
        );
        assert_eq!(state.error, "Worker không khả dụng. Hãy mở lại ứng dụng.");
    }

    #[test]
    fn details_hide_interface_path_and_preserve_unknown_metadata() {
        let endpoint = crate::devices::types::synthetic_endpoint("synthetic://private-serial-1234");
        let group = DeviceGroup {
            id: GroupId::Interface(endpoint.path.clone()),
            interfaces: vec![endpoint],
        };
        let detail = details(&group);
        assert!(!detail.contains("private-serial-1234"));
        assert!(detail.contains("Tên: Chưa biết"));
        assert!(detail.contains("Hãng: Chưa biết"));
        assert!(detail.contains("Chưa hỗ trợ điều khiển"));
        assert!(group_label(&group).contains("Interface chưa ghép"));
    }
}
