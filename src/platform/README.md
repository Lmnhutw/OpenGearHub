# Tích hợp hệ điều hành

Chỗ dành cho API Windows khi công việc đang làm cần đến: identity/container, device notification hoặc Core Audio.

- Giai đoạn 1 có thể thêm `mod.rs` và `windows.rs` khi cần thông tin identity; audio thuộc giai đoạn 4.
- Cô lập FFI và kiểu hệ điều hành; trả dữ liệu Rust sở hữu độc lập cho tầng trên.
- Giữ `unsafe` ở phạm vi nhỏ và giải thích invariant cần thiết.
- Xử lý lỗi quyền truy cập và đóng handle/callback; không mặc định yêu cầu admin.
- Chỉ tạo adapter/trait khi có nhu cầu thực, không scaffold Linux/macOS.

Xem [kiến trúc](../../docs/architecture.md).
