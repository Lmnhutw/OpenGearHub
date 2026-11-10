#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::container_id;

#[cfg(not(windows))]
pub fn container_id(_path: &str) -> Option<[u8; 16]> {
    None
}
