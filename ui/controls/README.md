# Control theo capability

Chỗ dành cho component Slint dùng chung như thông tin pin, điều khiển DPI hoặc RGB khi tính năng tương ứng được triển khai.

- Nhận dữ liệu/giới hạn từ capability; gửi thao tác qua callback có kiểu ở host.
- Thêm model có capability đã có phải tái sử dụng control hiện tại.
- Không tự kiểm tra tên hãng/model và không chạy I/O.
- Không hiển thị dữ liệu giả như pin 0% khi chưa đọc được.

Chưa có control nào được triển khai. Xem [kiến trúc](../../docs/architecture.md).
