use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use super::types::DeviceGroup;

pub struct ScanResult {
    pub inventory: Result<Vec<DeviceGroup>, String>,
    pub elapsed: Duration,
}

pub struct Worker {
    commands: Option<SyncSender<()>>,
    results: Receiver<ScanResult>,
    handle: Option<JoinHandle<()>>,
    closed: Arc<AtomicBool>,
    pending: bool,
}

impl Worker {
    pub fn spawn(
        mut discover: impl FnMut() -> Result<Vec<DeviceGroup>, String> + Send + 'static,
        notify: impl Fn() + Send + 'static,
    ) -> std::io::Result<Self> {
        let (commands, requests) = mpsc::sync_channel(1);
        let (send_results, results) = mpsc::sync_channel(1);
        let closed = Arc::new(AtomicBool::new(false));
        let closing = Arc::clone(&closed);
        let handle = thread::Builder::new()
            .name("hid-inventory".into())
            .spawn(move || {
                while requests.recv().is_ok() {
                    if closing.load(Ordering::Acquire) {
                        break;
                    }
                    let started = Instant::now();
                    let inventory =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut discover))
                            .unwrap_or_else(|_| {
                                Err("Tiến trình quét gặp lỗi nội bộ. Hãy thử làm mới.".into())
                            });
                    if closing.load(Ordering::Acquire) {
                        break;
                    }
                    if send_results
                        .try_send(ScanResult {
                            inventory,
                            elapsed: started.elapsed(),
                        })
                        .is_err()
                    {
                        break;
                    }
                    notify();
                }
            })?;
        Ok(Self {
            commands: Some(commands),
            results,
            handle: Some(handle),
            closed,
            pending: false,
        })
    }

    pub fn refresh(&mut self) -> Result<bool, String> {
        if self.closed.load(Ordering::Acquire) {
            return Err("Worker đã đóng.".into());
        }
        if self.pending {
            return Ok(false);
        }
        match self
            .commands
            .as_ref()
            .ok_or("Worker không khả dụng.")?
            .try_send(())
        {
            Ok(()) => {
                self.pending = true;
                Ok(true)
            }
            Err(TrySendError::Full(())) => Ok(false),
            Err(TrySendError::Disconnected(())) => Err("Không thể kết nối worker quét HID.".into()),
        }
    }

    pub fn try_result(&mut self) -> Result<Option<ScanResult>, String> {
        match self.results.try_recv() {
            Ok(result) => {
                self.pending = false;
                Ok(Some(result))
            }
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => {
                self.pending = false;
                Err("Worker quét HID đã kết thúc.".into())
            }
        }
    }

    pub fn shutdown(&mut self) {
        self.closed.store(true, Ordering::Release);
        self.commands.take();
        if let Some(handle) = self.handle.take()
            && handle.is_finished()
        {
            let _ = handle.join();
        }
        // shortcut: Windows enumeration cannot be cancelled; unfinished read-only work detaches on exit.
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_refresh_does_not_queue_more_scans() {
        let (entered, started) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        let (notified, finished) = mpsc::channel();
        let mut worker = Worker::spawn(
            move || {
                entered.send(()).unwrap();
                blocked.recv().unwrap();
                Ok(vec![])
            },
            move || {
                notified.send(()).unwrap();
            },
        )
        .unwrap();
        assert!(worker.refresh().unwrap());
        started.recv_timeout(Duration::from_secs(5)).unwrap();
        for _ in 0..20 {
            assert!(!worker.refresh().unwrap());
        }
        release.send(()).unwrap();
        finished.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(
            worker
                .try_result()
                .unwrap()
                .unwrap()
                .inventory
                .unwrap()
                .is_empty()
        );
        assert!(worker.refresh().unwrap());
        started.recv_timeout(Duration::from_secs(5)).unwrap();
        for _ in 0..20 {
            assert!(!worker.refresh().unwrap());
        }
        release.send(()).unwrap();
        finished.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(
            worker
                .try_result()
                .unwrap()
                .unwrap()
                .inventory
                .unwrap()
                .is_empty()
        );
        worker.shutdown();
        assert!(matches!(
            started.recv_timeout(Duration::from_secs(5)),
            Err(mpsc::RecvTimeoutError::Disconnected)
        ));
    }

    #[test]
    fn closing_blocked_discovery_does_not_join_or_publish_late_result() {
        let (entered, started) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        let (notified, events) = mpsc::channel();
        let mut worker = Worker::spawn(
            move || {
                entered.send(()).unwrap();
                blocked.recv().unwrap();
                Ok(vec![])
            },
            move || {
                notified.send(()).unwrap();
            },
        )
        .unwrap();
        worker.refresh().unwrap();
        started.recv_timeout(Duration::from_secs(5)).unwrap();
        worker.shutdown();
        assert!(worker.refresh().is_err());
        release.send(()).unwrap();
        assert!(matches!(
            events.recv_timeout(Duration::from_secs(5)),
            Err(mpsc::RecvTimeoutError::Disconnected)
        ));
        assert_eq!(
            worker.try_result().err().as_deref(),
            Some("Worker quét HID đã kết thúc.")
        );
    }

    #[test]
    fn discovery_error_and_panic_return_failures_instead_of_stuck_loading() {
        let (sent, received) = mpsc::channel();
        let mut worker = Worker::spawn(
            || Err("synthetic discovery error".into()),
            move || {
                sent.send(()).unwrap();
            },
        )
        .unwrap();
        worker.refresh().unwrap();
        received.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(
            worker.try_result().unwrap().unwrap().inventory.unwrap_err(),
            "synthetic discovery error"
        );
        let (sent, received) = mpsc::channel();
        let mut panicking = Worker::spawn(
            || panic!("synthetic panic"),
            move || {
                sent.send(()).unwrap();
            },
        )
        .unwrap();
        panicking.refresh().unwrap();
        received.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(
            panicking
                .try_result()
                .unwrap()
                .unwrap()
                .inventory
                .unwrap_err()
                .contains("lỗi nội bộ")
        );
    }
}
