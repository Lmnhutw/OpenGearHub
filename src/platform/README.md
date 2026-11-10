# Tích hợp hệ điều hành

Đã triển khai lookup Container ID từ device interface path bằng Configuration Manager API trong `windows.rs`. Kết quả là dữ liệu Rust sở hữu độc lập; kiểu FFI không đi vào app/UI.

- Lookup đọc instance ID rồi Container ID; validate kiểu/kích thước property và từ chối GUID rỗng.
- Metadata không có, buffer không đủ hoặc lookup thất bại trả về `None`; interface được giữ riêng thay vì tự gộp theo VID/PID.
- Không mở HidDevice hoặc gửi report để lấy Container ID. hidapi tự mở handle metadata quyền truy cập 0 trong enumeration.
- Cô lập FFI và kiểu hệ điều hành; trả dữ liệu Rust sở hữu độc lập cho tầng trên.
- Giữ `unsafe` ở phạm vi nhỏ và giải thích invariant cần thiết.
- Xử lý lỗi quyền truy cập và đóng handle/callback; không mặc định yêu cầu admin.
- Notification/audio và adapter Linux/macOS chưa được triển khai.

Xem [kiến trúc](../../docs/architecture.md).
