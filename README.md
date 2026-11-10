# OpenGearHub

Ứng dụng desktop mã nguồn mở bằng Rust để quản lý thiết bị ngoại vi nhiều hãng trong một giao diện. GearHub là tên hiển thị đang được cân nhắc.

**Trạng thái hiện tại:** đã có khung thư mục và tài liệu định hướng; chưa có ứng dụng chạy được, Cargo project hoặc model thiết bị được kiểm chứng.

## Bắt đầu từ đâu

| Tài liệu | Nội dung |
| --- | --- |
| [PROJECT.md](PROJECT.md) | Mục tiêu, scope, quyết định và thông tin còn thiếu |
| [Kiến trúc](docs/architecture.md) | Cấu trúc code, trách nhiệm module và cơ chế mở rộng thiết bị |
| [Roadmap và tiến độ](docs/roadmap.md) | Công việc theo giai đoạn, điều kiện hoàn thành và lịch sử thực hiện |
| [Thêm thiết bị](docs/adding-device.md) | Quy trình thêm model, protocol hoặc capability |
| [Thiết bị hỗ trợ](docs/supported-devices.md) | Bằng chứng tương thích theo model và kết nối |
| [Kiểm chứng](docs/verification.md) | Build/test, UI, phần cứng và đo tài nguyên |
| [AGENTS.md](AGENTS.md) | Hướng dẫn cho các phiên làm việc với coding agent |

Đọc `PROJECT.md` trước, sau đó chọn công việc tiếp theo trong roadmap. Hướng dẫn build/run sẽ được bổ sung khi hoàn thành bước khởi tạo Cargo ở giai đoạn 1.

## License

Source của dự án hiện theo [MIT](LICENSE). Việc lựa chọn license của Slint và nghĩa vụ khi phân phối binary được theo dõi riêng trong `PROJECT.md`.
