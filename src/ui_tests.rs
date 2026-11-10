//! Functional tests of the real Slint component with synthetic data; no screenshots or hardware I/O.

use std::{cell::Cell, rc::Rc, time::Duration};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, ModelRc, PhysicalSize, StandardListViewItem, VecModel};

use crate::AppWindow;

struct TestPlatform(Rc<MinimalSoftwareWindow>);

impl Platform for TestPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.0.clone())
    }

    fn duration_since_start(&self) -> Duration {
        Duration::ZERO
    }
}

fn press_key(ui: &AppWindow, key: Key) {
    let text: slint::SharedString = key.into();
    ui.window()
        .dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    ui.window()
        .dispatch_event(WindowEvent::KeyReleased { text });
}

#[test]
fn app_window_handles_keyboard_selection_and_refresh_gating_at_both_sizes() {
    std::thread::Builder::new()
        .name("functional-ui-test".into())
        .spawn(|| {
            let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
            slint::platform::set_platform(Box::new(TestPlatform(window))).unwrap();
            let ui = AppWindow::new().unwrap();
            let refresh_count = Rc::new(Cell::new(0));
            let callback_count = Rc::clone(&refresh_count);
            ui.on_refresh(move || callback_count.set(callback_count.get() + 1));
            let selection = Rc::new(Cell::new(-1));
            let callback_selection = Rc::clone(&selection);
            ui.on_selection_changed(move |index| callback_selection.set(index));
            ui.show().unwrap();
            ui.window()
                .dispatch_event(WindowEvent::WindowActiveChanged(true));

            for (width, height) in [(1100, 720), (800, 520)] {
                ui.window().set_size(PhysicalSize::new(width, height));
                slint::platform::update_timers_and_animations();
                assert_eq!(ui.window().size(), PhysicalSize::new(width, height));
                ui.set_devices(ModelRc::new(VecModel::from(vec![
                    StandardListViewItem::from("SYNTHETIC · Interface chưa ghép"),
                    StandardListViewItem::from("SYNTHETIC · Nhóm Windows"),
                ])));
                ui.set_selected_index(0);
                press_key(&ui, Key::DownArrow);
                assert_eq!(
                    ui.get_selected_index(),
                    1,
                    "arrow navigation must select the next row"
                );
                assert_eq!(selection.get(), 1, "selection must reach the host callback");

                ui.set_refreshing(false);
                let before = refresh_count.get();
                press_key(&ui, Key::F5);
                assert_eq!(
                    refresh_count.get(),
                    before + 1,
                    "idle F5 must request a refresh"
                );
                ui.set_refreshing(true);
                press_key(&ui, Key::F5);
                assert_eq!(
                    refresh_count.get(),
                    before + 1,
                    "busy F5 must not enqueue another refresh"
                );
                ui.set_refreshing(false);
                press_key(&ui, Key::F5);
                assert_eq!(
                    refresh_count.get(),
                    before + 2,
                    "F5 must work again after a scan"
                );
            }
            ui.hide().unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}
