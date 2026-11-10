# OpenGearHub

Ứng dụng desktop mã nguồn mở bằng Rust để quản lý thiết bị ngoại vi nhiều hãng trong một giao diện. GearHub là tên hiển thị đang được cân nhắc.

**Trạng thái hiện tại:** đã triển khai nền tảng Phase 1: Cargo project, giao diện Slint và inventory HID chỉ đọc trên Windows. Chưa có model được xác minh điều khiển. Kết quả build/test, UI và phần cứng được ghi riêng trong [kiểm chứng](docs/verification.md) và [roadmap](docs/roadmap.md).

## Build và chạy trên Windows

Target hiện tại là `x86_64-pc-windows-msvc`, cần Rust cùng MSVC Build Tools và Windows SDK. Rust 1.96.0 đã build/test qua, MSRV 1.92.0 đã check qua. [Báo cáo Phase 1](docs/phase-1-report.md) phân biệt kết quả tự động, kiểm tra tay và baseline.

```powershell
cargo run --locked --target x86_64-pc-windows-msvc
cargo fmt --all -- --check
cargo check --locked --target x86_64-pc-windows-msvc
cargo clippy --locked --all-targets --target x86_64-pc-windows-msvc
cargo test --locked --target x86_64-pc-windows-msvc
cargo build --release --locked --target x86_64-pc-windows-msvc
```

Dependency trực tiếp: Slint/slint-build 1.18.1, hidapi 2.6.7 với `windows-native`, windows-sys 0.61. Slint dùng Winit + software renderer và accessibility; chưa bật tray hoặc renderer GPU.

## Sử dụng và giới hạn hiện tại

- App quét khi mở; dùng **Làm mới**, menu **Thiết bị → Làm mới** hoặc **F5** để quét lại. Chưa có tự cập nhật khi cắm/rút.
- Chọn một hàng bằng chuột/bàn phím để xem metadata của các interface. Hàng có Container ID được ghi là **Nhóm Windows**; interface chưa có identity nhóm được giữ riêng.
- Nhóm Windows không chứng minh một thiết bị ngoại vi độc lập: receiver có thể đại diện nhiều thiết bị logic. Hai model cùng VID/PID không tự bị gộp.
- Chỉ enumerate HID; không đọc input report/phím gõ hoặc gửi lệnh điều khiển riêng hãng. Pin, DPI, RGB, audio, profile và tray chưa được triển khai.
- Metadata có thể thiếu; backend có thể bỏ qua interface không mở được để đọc metadata. Không coi danh sách là toàn bộ thiết bị ngoại vi hoặc bằng chứng quyền điều khiển.
- Refresh lỗi giữ inventory trước cùng thông báo dữ liệu chưa cập nhật. Raw device path và serial không được hiển thị trong chi tiết.
- Discovery chạy ngoài UI thread. Windows enumeration không có cancellation trong backend hiện tại; khi thoát, worker chưa hoàn tất được detach và không publish kết quả muộn. Chưa bảo đảm hủy tức thời lời gọi hệ điều hành.
- Menu **Trợ giúp → Giới thiệu OpenGearHub** hiển thị widget `AboutSlint`.

## Bắt đầu từ đâu

| Tài liệu | Nội dung |
| --- | --- |
| [PROJECT.md](PROJECT.md) | Mục tiêu, scope, quyết định và thông tin còn thiếu |
| [Kiến trúc](docs/architecture.md) | Cấu trúc code, trách nhiệm module và cơ chế mở rộng thiết bị |
| [Roadmap và tiến độ](docs/roadmap.md) | Công việc theo giai đoạn, điều kiện hoàn thành và lịch sử thực hiện |
| [Thêm thiết bị](docs/adding-device.md) | Quy trình thêm model, protocol hoặc capability |
| [Thiết bị hỗ trợ](docs/supported-devices.md) | Bằng chứng tương thích theo model và kết nối |
| [Kiểm chứng](docs/verification.md) | Build/test, UI, phần cứng và đo tài nguyên |
| [Báo cáo Phase 1](docs/phase-1-report.md) | 29 test, inventory thật, baseline và giới hạn |
| [AGENTS.md](AGENTS.md) | Hướng dẫn cho các phiên làm việc với coding agent |
| [Third-party notices](THIRD_PARTY_NOTICES.md) | License được chọn và điều kiện phân phối dependency |

Đọc `PROJECT.md` trước, sau đó chọn công việc tiếp theo trong roadmap. Các lệnh trên là hướng dẫn; bằng chứng check đã chạy nằm trong tài liệu kiểm chứng.

## License

Source của dự án giữ [MIT](LICENSE). Slint được sử dụng theo **Royalty-free 2.0**, với `AboutSlint` truy cập từ menu cấp cao nhất. Xem [third-party notices](THIRD_PARTY_NOTICES.md); rà đầy đủ dependency bắc cầu, asset và notice của binary vẫn là công việc trước phát hành.
