# Protocol implementations

Chỗ dành cho bộ xử lý giao thức thật từ giai đoạn 2. Hiện chưa có implementation.

- Tổ chức theo họ giao thức, chia sẻ cho các model đã xác minh tương thích.
- Tách encode/decode report khỏi I/O để kiểm thử bằng fixture.
- Validate lệnh và phản hồi; ghi cách xác nhận kết quả, timeout và hạn chế của API/backend.
- Không phụ thuộc Slint hoặc app state; không dò thiết bị bằng report tùy ý.
- Thêm protocol qua registry và contract driver phù hợp, không thêm nhánh riêng vào worker chung.

Xem [kiến trúc](../../../docs/architecture.md) và [adding-device](../../../docs/adding-device.md).
