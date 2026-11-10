# OpenGearHub — Hồ sơ dự án

Cập nhật: 2026-11-10. Trạng thái triển khai và công việc tiếp theo được quản lý tại [roadmap](docs/roadmap.md).

## 1. Mục tiêu

Xây dựng ứng dụng desktop bằng Rust, giúp người dùng quản lý thiết bị ngoại vi nhiều hãng trong một giao diện. Ưu tiên ổn định, tiết kiệm tài nguyên, dễ đọc code và dễ thêm thiết bị qua đóng góp cộng đồng.

Ứng dụng hoạt động cục bộ, không cần tài khoản hoặc backend máy chủ. Chỉ công bố tính năng và thiết bị đã có bằng chứng phù hợp; không hứa thay thế toàn bộ phần mềm của mọi hãng.

## 2. Định hướng đang dùng

| Hạng mục | Định hướng | Điều cần xác minh |
| --- | --- | --- |
| Ngôn ngữ | Rust, edition 2024 | Toolchain và MSRV theo toàn bộ dependency được chọn |
| Nền tảng | Windows-first, dự kiến Windows x64 | Phiên bản Windows tối thiểu và môi trường build thực tế |
| UI | Slint, không dùng WebView | Phiên bản, backend, renderer, accessibility và license phân phối |
| Kiến trúc | Một executable, một Cargo package, module theo trách nhiệm | Kiểm chứng ranh giới bằng hai model của hai hãng |
| Mở rộng | Registry chọn bộ xử lý theo model/interface; UI theo capability | Model mới cần source code và build lại |
| HID | Dự kiến dùng hidapi | Backend/features Windows và quyền truy cập interface thực |
| Windows API | Thêm khi cần identity, notification hoặc audio | Feature flags tối thiểu cho tính năng đang làm |
| License source | Giữ MIT hiện có | Rà nghĩa vụ dependency và binary trước khi phát hành |

Slint 1.18.1 là phiên bản tham chiếu khi lập kế hoạch và yêu cầu Rust từ 1.92. Đây chưa phải dependency đã được cài hoặc khóa trong repo. Khi triển khai, kiểm tra lại phiên bản, MSRV, features và license; đồng bộ `slint`/`slint-build`, commit `Cargo.lock`.

## 3. Scope theo mốc sản phẩm

### Giai đoạn 1: nền tảng và nhận diện HID

- Tạo app Rust + Slint tối giản, discovery ngoài UI thread.
- Liệt kê HID thật, xem chi tiết, chọn thiết bị và refresh thủ công.
- Phân biệt thiết bị vật lý, interface và trường hợp chưa đủ dữ liệu để nhóm.
- Có loading, empty, error, unsupported và dữ liệu chưa biết.
- Chuẩn bị ranh giới discovery, registry, worker và UI theo [kiến trúc](docs/architecture.md).
- Thêm kiểm tra tự động không cần phần cứng; đo baseline release build.
- Không gửi report điều khiển riêng của hãng. Chưa thêm audio, profile hoặc tray.

### Giai đoạn 2: một model, một tính năng thật

Chọn model người dùng có để kiểm thử, xác minh giao thức và interface. Đọc trạng thái trước, sau đó bổ sung một tính năng phù hợp như DPI hoặc RGB. Bộ xử lý phải validate lệnh, xử lý lỗi và nêu rõ cách xác nhận kết quả.

### Giai đoạn 3: chứng minh quản lý đa hãng

Thêm model của hãng thứ hai. Kiểm thử hai thiết bị hoạt động đồng thời, định tuyến lệnh đúng thiết bị và khả năng tái sử dụng UI/capability. **MVP quản lý đa hãng hoàn thành ở mốc này**, sau khi có bằng chứng phần cứng.

### Giai đoạn 4: tiện ích hằng ngày và phát hành

Lần lượt bổ sung audio endpoint/volume/mute, profile, tray và quy trình phát hành. Đổi thiết bị âm thanh mặc định là hạng mục cần xác minh riêng. Mỗi tiện ích là một công việc riêng có tiêu chí kiểm chứng.

Chi tiết nhiệm vụ và điều kiện kết thúc từng giai đoạn nằm trong [roadmap](docs/roadmap.md).

## 4. Yêu cầu xuyên suốt

- **Mở rộng:** model mapping, protocol và capability có trách nhiệm riêng. Model mới dùng capability đã có không cần sửa UI hoặc worker chung.
- **Nhận diện:** VID/PID là dữ kiện, không phải identity duy nhất hoặc bằng chứng hỗ trợ giao thức. Model, interface, kiểu kết nối và firmware cần được xét khi có ảnh hưởng.
- **Trung thực:** detected, đọc trạng thái và điều khiển là ba mức khác nhau. Không có dữ liệu pin thì thể hiện chưa có dữ liệu, không dùng 0% thay thế.
- **Độ tin cậy:** không chạy I/O blocking trên UI thread; hàng đợi có giới hạn; không giữ lock khi I/O; xử lý disconnect và kết thúc worker rõ ràng.
- **An toàn thiết bị:** chỉ thực thi lệnh đã biết trên model/interface phù hợp. Không dò bằng report tùy ý, flash firmware, thay driver hoặc tự kill app hãng.
- **Quyền truy cập:** không mặc định chạy app bằng admin. Thông báo rõ lỗi truy cập hoặc xung đột với phần mềm khác.
- **Dữ liệu:** khi có config, ghi thay thế an toàn theo Windows; không tự ghi đè file hỏng hoặc xóa dữ liệu để sửa lỗi.
- **Riêng tư:** không thu thập phím gõ, telemetry hay tự tải log lên mạng. Làm sạch serial, đường dẫn cá nhân và dữ liệu nhạy cảm trong log/fixture chia sẻ.
- **UI:** dùng capability và giới hạn thực tế; hỗ trợ bàn phím, resize và trạng thái lỗi. Chỉ ghi đã kiểm tra giao diện khi đã xem render.
- **Hiệu năng:** đo startup, bộ nhớ và CPU idle bằng release build; ghi máy, phiên bản, số thiết bị và cách đo. Chốt budget sau khi có baseline.

## 5. Ngoài scope hiện tại

Cloud/account, đồng bộ online, database, web server, Windows service, firmware updater, custom kernel driver, macro ghi toàn bộ phím, plugin tải từ bên ngoài và Linux/macOS.

“Dynamic” hiện là runtime discovery, chọn bộ xử lý trong registry và hiển thị capability. Chưa có cơ chế hot-load DLL/script hoặc thêm giao thức tùy ý chỉ bằng file JSON.

## 6. Những thông tin còn thiếu

| Quyết định | Trạng thái | Cần trước mốc |
| --- | --- | --- |
| Model đầu tiên, USB/Bluetooth/receiver, firmware nếu biết | Chưa có dữ liệu | Triển khai protocol ở giai đoạn 2 |
| Model của hãng thứ hai và thiết bị kiểm thử | Chưa có dữ liệu | Giai đoạn 3 |
| Windows tối thiểu, Rust/MSVC/Windows SDK và dependency cụ thể | Chưa kiểm chứng | Khởi tạo app ở giai đoạn 1 |
| Ngân sách RAM/CPU/startup | Chưa có baseline | Đánh giá hiệu năng sau baseline giai đoạn 1 |
| Lựa chọn license Slint và notice khi phân phối | Chưa chốt | Chọn điều khoản phù hợp khi tích hợp; rà lại trước phát hành |
| Tên hiển thị chính thức OpenGearHub/GearHub | Tên làm việc | Trước phát hành công khai |

Thiếu model kiểm thử không chặn công việc nhận diện an toàn ở giai đoạn 1.

## 7. Cách duy trì bộ tài liệu

- `PROJECT.md`: mục tiêu, scope, quyết định và các câu hỏi còn mở.
- `docs/architecture.md`: ranh giới module, luồng dữ liệu và cấu trúc đích.
- `docs/roadmap.md`: nguồn theo dõi trạng thái nhiệm vụ; cập nhật bằng bằng chứng thực.
- `docs/supported-devices.md`: model và mức xác minh; không đánh dấu hỗ trợ cả hãng.
- `docs/verification.md`: cách kiểm tra và ghi kết quả.
- `docs/adding-device.md`: quy trình đóng góp thiết bị.

Khi kết thúc một công việc, cập nhật nhiệm vụ tương ứng trong roadmap, ghi check đã chạy/chưa chạy và bước tiếp theo. Thay đổi quyết định kiến trúc phải cập nhật tài liệu liên quan trong cùng thay đổi.

## 8. Nguồn đối chiếu

- [Slint: cài đặt và yêu cầu Rust](https://docs.slint.dev/docs/slint/guide/getting-started/)
- [Slint 1.18.1: các lựa chọn license](https://github.com/slint-ui/slint/blob/v1.18.1/LICENSE.md)
- [hidapi Rust API](https://docs.rs/hidapi/)
- [Windows Container ID](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/container-ids)
- [Windows keyboard/mouse HID clients](https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/keyboard-and-mouse-hid-client-drivers)

Đối chiếu tài liệu đúng phiên bản khi triển khai; danh sách trên không phải dependency đã tích hợp hoặc chứng nhận tương thích.
