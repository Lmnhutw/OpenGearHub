# Model specifications

Chỗ dành cho mapping và giới hạn của model được triển khai từ giai đoạn 2. Hiện chưa có model được hỗ trợ.

- Mỗi specification mô tả dữ kiện nhận diện, interface/connection, protocol và giới hạn có bằng chứng.
- Không suy ra quyền điều khiển chỉ từ tên hãng hoặc VID/PID.
- Không chứa I/O, USB report hoặc code UI; nhiều model có thể dùng chung protocol.
- Thêm mapping phải kèm test nhận diện và cập nhật mức tương thích.

Làm theo [adding-device](../../../docs/adding-device.md) và đăng ký qua registry chung khi triển khai.
