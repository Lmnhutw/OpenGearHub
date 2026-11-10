# Roadmap và tiến độ

Cập nhật: 2026-11-10. Đây là nguồn theo dõi trạng thái nhiệm vụ. Scope ở [PROJECT.md](../PROJECT.md), cách kiểm tra ở [verification](verification.md).

## Trạng thái hiện tại

- Đã hoàn thành mốc chuẩn bị: khung thư mục và tài liệu (G0).
- Phase 1 đã triển khai và kiểm chứng: 29 test qua, build release/MSRV qua, inventory thật và kiểm tra tay do người dùng xác nhận. Xem [báo cáo](phase-1-report.md).
- Công việc tiếp theo khi được giao giai đoạn mới: **G2-01**. Chưa có model điều khiển nào được hỗ trợ; CI remote chưa chạy.
- Chưa có model kiểm thử; thông tin này chỉ cần trước phần protocol ở giai đoạn 2.

Chỉ đánh dấu `[x]` khi có bằng chứng hoàn thành. Khi đang làm, thêm dòng `Đang làm: <ID>`; khi bị chặn, ghi nguyên nhân và phần vẫn có thể tiếp tục. Không tự chuyển giai đoạn từ danh sách bên dưới.

## G0 — Khung và tài liệu

- [x] **G0-01:** Chốt tài liệu mục tiêu, scope, kiến trúc mở rộng và trách nhiệm thư mục.
- [x] **G0-02:** Tạo thư mục source/UI/fixture với README hướng dẫn, không giả lập implementation.
- [x] **G0-03:** Tạo quy trình thêm thiết bị, bảng tương thích và checklist kiểm chứng.
- [x] **G0-04:** Kiểm tra liên kết Markdown, cấu trúc thư mục, whitespace và LICENSE không thay đổi.

## G1 — Nền tảng và nhận diện HID

- [x] **G1-01:** Kiểm tra Rust, target Windows, MSVC/Windows SDK; chọn dependency, features và điều khoản Slint phù hợp.
- [x] **G1-02:** Khởi tạo một Cargo package và cửa sổ Slint; tạo Cargo.lock để đưa vào Git, ignore build output và hướng dẫn build/run. Changeset chưa commit/push.
- [x] **G1-03:** Descriptor/identity, discovery HID và chuẩn hóa dữ liệu thiếu ngoài UI thread; quan sát 16 interface / 3 nhóm.
- [x] **G1-04:** Matcher/registry và test match mơ hồ; production registry rỗng, không có protocol giả.
- [x] **G1-05:** Worker, refresh không chặn UI, bỏ yêu cầu lặp và kiểm tra đóng khi scan đang bị chặn.
- [x] **G1-06:** UI danh sách–chi tiết và trạng thái; test bàn phím/F5/resize, người dùng xác nhận thao tác native.
- [x] **G1-07:** Test nhiều interface, cùng VID/PID, metadata thiếu và discovery lỗi; dữ liệu synthetic ghi rõ nguồn gốc.
- [x] **G1-08:** Tạo Windows CI; các lệnh tương ứng qua local; người dùng xác nhận refresh/cắm-rút/thoát app. CI trên GitHub chưa chạy.
- [x] **G1-09:** Baseline release một mẫu, kết quả và giới hạn ghi tại báo cáo Phase 1.

Điều kiện hoàn thành: inventory HID thật có identity trung thực, UI đáp ứng khi discovery chậm và kết quả shutdown được ghi nhận. Không gửi report điều khiển riêng hãng. Chưa thêm audio, profile hoặc tray. Sau báo cáo G1, chờ yêu cầu tiếp theo.

## G2 — Một model, một tính năng thật

- [ ] **G2-01:** Ghi model, connection, firmware nếu biết, interface và nguồn giao thức đáng tin cậy.
- [ ] **G2-02:** Hoàn thiện contract driver tối thiểu; đọc trạng thái và xử lý report thiếu/sai bằng test.
- [ ] **G2-03:** Thêm một điều khiển phù hợp, capability/giới hạn, validation và tiêu chí xác nhận kết quả.
- [ ] **G2-04:** Kiểm thử disconnect/reconnect, timeout, giá trị ngoài giới hạn, lệnh cũ và xung đột truy cập.
- [ ] **G2-05:** Cập nhật bảng tương thích bằng kết quả phần cứng và báo cáo giới hạn.

Điều kiện hoàn thành: một tính năng thực được kiểm chứng trên đúng model/kết nối, có cách phân biệt pending, thành công, chưa xác nhận và thất bại. Thiếu giao thức hoặc thiết bị kiểm thử thì không gửi lệnh đoán.

## G3 — Chứng minh quản lý đa hãng

- [ ] **G3-01:** Chọn model hãng thứ hai và ghi dữ kiện nhận diện/giao thức.
- [ ] **G3-02:** Thêm mapping/protocol bằng điểm mở rộng hiện có; tái sử dụng capability phù hợp.
- [ ] **G3-03:** Kiểm thử hai thiết bị đồng thời, định tuyến đúng, xử lý một thiết bị lỗi và hai thiết bị cùng model khi có điều kiện.
- [ ] **G3-04:** Rà ranh giới: model dùng capability đã có không yêu cầu sửa UI hoặc worker chung; cập nhật tài liệu và bằng chứng.

Điều kiện hoàn thành: có model của hai hãng với khả năng điều khiển được kiểm chứng và hoạt động cùng nhau. Đây là mốc MVP quản lý đa hãng.

## G4 — Tiện ích hằng ngày và phát hành

Chọn từng công việc sau theo yêu cầu thực; không triển khai tất cả trong một lượt.

- [ ] **G4-01:** Audio endpoint, volume/mute và notification; xác minh riêng khả năng đổi endpoint mặc định nếu được yêu cầu.
- [ ] **G4-02:** Profile cho capability đã hỗ trợ; lưu config an toàn, phục hồi file hỏng và báo lỗi từng mục khi áp dụng.
- [ ] **G4-03:** Tray; phân biệt đóng xuống tray với thoát hoàn toàn; đo lại CPU/bộ nhớ khi ẩn.
- [ ] **G4-04:** Rà license/notice, đóng gói Windows, hướng dẫn cài/chạy, issue template, contribution và release checklist.
- [ ] **G4-05:** Đo lại tài nguyên với cùng phương pháp baseline, ghi regression và kết quả phần cứng cho bản phát hành.

## Nhật ký thực hiện

| Ngày | Công việc | Kết quả/bằng chứng | Bước tiếp theo |
| --- | --- | --- | --- |
| 2026-11-10 | G0-01 đến G0-04 | 15 file Markdown, 37 link nội bộ và các thư mục dự kiến đã kiểm tra; whitespace hợp lệ, LICENSE không đổi. Chưa có Cargo project hoặc kiểm thử app/phần cứng | G1-01 khi được giao triển khai |
| 2026-11-10 | G1-01 đến G1-09 | 29 test qua; fmt/check/clippy/release/MSRV qua; inventory 16/3 → 10/2 → 16/3, process thoát exit 0 và người dùng xác nhận kiểm tra tay. [Chi tiết](phase-1-report.md) | G2-01 khi có yêu cầu tiếp theo; CI remote sau khi push |

Khi cập nhật nhật ký, dẫn tới task/PR/commit hoặc bản ghi kiểm chứng nếu có. Không chép log chứa dữ liệu cá nhân vào tài liệu.
