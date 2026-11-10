# Kiểm chứng và ghi nhận kết quả

Tài liệu này là checklist cho các giai đoạn trong [roadmap](roadmap.md), không phải kết quả test đã chạy. Hiện chưa có Cargo project, UI hoặc kiểm thử phần cứng.

## 1. Build và kiểm tra tự động

Chạy khi đã khởi tạo Cargo, tạo lockfile và cài target/toolchain phù hợp. Các lệnh dưới đây dự kiến dùng cho Windows x64 MSVC; điều chỉnh khi target được xác nhận.

```powershell
cargo fmt --all -- --check
cargo check --locked --target x86_64-pc-windows-msvc
cargo clippy --locked --all-targets --target x86_64-pc-windows-msvc
cargo test --locked --target x86_64-pc-windows-msvc
cargo build --release --locked --target x86_64-pc-windows-msvc
```

CI kiểm chứng build/logic không cần phần cứng. Ghi toolchain, target, command, exit status, warning liên quan và nguyên nhân check bị chặn. Không cần chạy test app cho thay đổi thuần Markdown; kiểm tra link và cấu trúc thay vào đó.

## 2. Logic không cần phần cứng

Chỉ thêm test khi logic tương ứng được triển khai.

- [ ] Nhiều interface không bị trình bày sai thành nhiều thiết bị vật lý đã xác định.
- [ ] Hai thiết bị cùng VID/PID không bị gộp thành một identity.
- [ ] Registry match đúng, không match và match mơ hồ.
- [ ] Metadata thiếu hoặc discovery lỗi không làm crash app.
- [ ] Encode/decode xử lý report hợp lệ, ngắn, sai định dạng và phản hồi chưa biết.
- [ ] Lệnh ngoài capability/giới hạn bị từ chối; lệnh của phiên kết nối cũ không đến thiết bị mới.
- [ ] Config hỏng và ghi thất bại không làm mất dữ liệu, khi có persistence.

## 3. UI và vòng đời

- [ ] Xem render thật và lưu ảnh/bằng chứng phù hợp sau thay đổi UI.
- [ ] Kiểm tra resize, overflow, focus, thao tác bàn phím và nhãn trạng thái.
- [ ] Kiểm tra loading, empty, error, unsupported và trường chưa có dữ liệu.
- [ ] Discovery chậm không chặn thao tác UI; refresh liên tiếp không tích lũy vô hạn.
- [ ] Cắm/rút rồi refresh cập nhật đúng danh sách và lựa chọn hiện tại.
- [ ] Thoát khi worker rảnh và khi đang I/O; ghi giới hạn nếu API không hủy được.
- [ ] Khi có tray: đóng xuống tray khác với thoát hoàn toàn, không để worker chạy sau thao tác thoát.

## 4. Phần cứng từ giai đoạn 2

- [ ] Ghi model, connection, firmware nếu biết, Windows, app revision và tính năng thử.
- [ ] Đọc trạng thái; kiểm tra cách thể hiện dữ liệu không có.
- [ ] Ghi giá trị hợp lệ, kiểm tra phản hồi/readback và giá trị biên.
- [ ] Disconnect/reconnect giữa tác vụ, timeout và xung đột truy cập app hãng.
- [ ] Khi có nhiều thiết bị: lệnh đến đúng đích; một thiết bị lỗi không làm sai trạng thái thiết bị khác.
- [ ] Ghi kết quả riêng cho từng capability và giới hạn xác nhận thực tế.

Không kiểm thử bằng report tùy ý, firmware flash, thay driver hoặc tự kill app hãng.

## 5. Baseline tài nguyên

Dùng release build; giữ phương pháp và môi trường giống nhau khi so sánh. Ghi cấu hình máy, Windows, app revision, backend/renderer, số thiết bị, công cụ đo và khoảng quan sát. Khởi động được tính từ mở process đến mốc UI/inventory được mô tả rõ.

| Kịch bản | Số liệu cần ghi | Trạng thái |
| --- | --- | --- |
| Khởi động | Thời gian đến UI và inventory đầu tiên | Chưa đo |
| UI mở, idle | CPU tiến trình, private bytes và working set | Chưa đo |
| Refresh | Thời gian discovery, CPU/bộ nhớ và độ đáp ứng UI | Chưa đo |
| UI ẩn/tray, khi đã có | CPU/bộ nhớ và tác vụ nền còn chạy | Chưa triển khai |

Không điền số giả hoặc cam kết budget trước baseline. Phân biệt working set với private bytes, không gọi chung một con số là toàn bộ RAM app.

## 6. Mẫu báo cáo công việc

```text
Ngày / task ID / app revision:
Mục tiêu và thay đổi:
Môi trường / phần cứng nếu có:
Check đã chạy và kết quả:
Check chưa chạy hoặc bị chặn, lý do:
Bằng chứng / giới hạn:
Tài liệu và mức hỗ trợ đã cập nhật:
Bước tiếp theo:
```

Ghi kết quả gắn với nhiệm vụ trong roadmap. Chỉ cập nhật bảng tương thích bằng bằng chứng phần cứng phù hợp.
