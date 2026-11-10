# Hướng dẫn làm việc với OpenGearHub

## Đọc trước khi thay đổi

1. Đọc [PROJECT.md](PROJECT.md), [roadmap](docs/roadmap.md) và [kiến trúc](docs/architecture.md).
2. Kiểm tra trạng thái Git và code thực tế. Tài liệu định hướng không thay thế bằng chứng trong repo.
3. Khi thêm thiết bị, đọc [adding-device](docs/adding-device.md) và bảng tương thích hiện có.

## Scope và tiến độ

- Thực hiện công việc/giai đoạn được người dùng giao. Roadmap mô tả công việc tương lai, không tự cấp quyền triển khai tất cả.
- Sau mỗi giai đoạn, báo thay đổi, kết quả kiểm chứng, giới hạn và công việc tiếp theo; chuyển giai đoạn khi có yêu cầu tiếp theo.
- Giữ nguyên thay đổi không liên quan và LICENSE MIT. Không tự đổi framework, license hoặc mở rộng nền tảng.
- Trước khi sửa code, xác định các caller, test, fixture, config, export và tài liệu chịu ảnh hưởng.

## Ranh giới code

- Giữ một executable/một Cargo package cho đến khi có lý do thực tế để tách.
- UI và app state không chứa USB report hay điều kiện riêng theo hãng/model.
- Registry là điểm đăng ký bộ xử lý. Model mapping là dữ liệu; protocol xử lý giao tiếp và encode/decode.
- Dùng command/result có kiểu. Chỉ thêm trait tại ranh giới cần thay thế implementation hoặc kiểm thử.
- Worker sở hữu handle; không chạy I/O blocking trên UI thread. Không gửi lệnh ghi cho thiết bị chưa hỗ trợ hoặc match chưa rõ ràng.
- Chỉ thêm dependency, file code và abstraction khi công việc đang thực hiện cần đến. README trong thư mục hiện là khung định hướng, không phải implementation.

## Kiểm chứng và cập nhật

- Theo [verification](docs/verification.md); không báo thành công cho check chưa chạy.
- Test giả lập không thay thế kiểm thử phần cứng. Không nâng mức hỗ trợ chỉ vì CI xanh.
- Ưu tiên test chức năng và tương tác. Chỉ chụp hoặc lưu ảnh màn hình khi người dùng yêu cầu rõ ràng; không coi ảnh bằng chứng là điều kiện mặc định để hoàn thành công việc.
- Cập nhật roadmap và tài liệu chịu ảnh hưởng trong cùng thay đổi. Ghi rõ blocker, dữ liệu còn thiếu và bước tiếp theo.
- Tài liệu dùng tiếng Việt hoặc tiếng Anh; identifier trong code giữ nhất quán theo Rust/Slint.
