# Roadmap và tiến độ

Cập nhật: 2026-11-10. Đây là nguồn theo dõi trạng thái nhiệm vụ. Scope ở [PROJECT.md](../PROJECT.md), cách kiểm tra ở [verification](verification.md).

## Trạng thái hiện tại

- Đã hoàn thành mốc chuẩn bị: khung thư mục và tài liệu (G0).
- Giai đoạn 1 chưa bắt đầu; chưa có Cargo project, app hoặc hỗ trợ thiết bị thực.
- Công việc tiếp theo khi được giao triển khai: **G1-01**.
- Chưa có model kiểm thử; thông tin này chỉ cần trước phần protocol ở giai đoạn 2.

Chỉ đánh dấu `[x]` khi có bằng chứng hoàn thành. Khi đang làm, thêm dòng `Đang làm: <ID>`; khi bị chặn, ghi nguyên nhân và phần vẫn có thể tiếp tục. Không tự chuyển giai đoạn từ danh sách bên dưới.

## G0 — Khung và tài liệu

- [x] **G0-01:** Chốt tài liệu mục tiêu, scope, kiến trúc mở rộng và trách nhiệm thư mục.
- [x] **G0-02:** Tạo thư mục source/UI/fixture với README hướng dẫn, không giả lập implementation.
- [x] **G0-03:** Tạo quy trình thêm thiết bị, bảng tương thích và checklist kiểm chứng.
- [x] **G0-04:** Kiểm tra liên kết Markdown, cấu trúc thư mục, whitespace và LICENSE không thay đổi.

## G1 — Nền tảng và nhận diện HID

- [ ] **G1-01:** Kiểm tra Rust, target Windows, MSVC/Windows SDK; chọn dependency, features và điều khoản Slint phù hợp.
- [ ] **G1-02:** Khởi tạo một Cargo package và cửa sổ Slint tối giản; commit lockfile, thêm ignore cho build output và cập nhật hướng dẫn build/run.
- [ ] **G1-03:** Thêm descriptor/identity, discovery HID và chuẩn hóa dữ liệu thiếu ngoài UI thread.
- [ ] **G1-04:** Thêm matcher/registry nhỏ với kiểm tra match mơ hồ; thiết bị chưa hỗ trợ chỉ được inventory. Không thêm protocol giả.
- [ ] **G1-05:** Thêm worker, refresh không chặn UI, gộp yêu cầu lặp và xử lý vòng đời.
- [ ] **G1-06:** UI danh sách–chi tiết, lựa chọn thiết bị, loading/empty/error/unsupported; kiểm tra resize và bàn phím.
- [ ] **G1-07:** Test nhiều interface, hai thiết bị cùng VID/PID, metadata thiếu và discovery lỗi; fixture ghi rõ nguồn gốc.
- [ ] **G1-08:** Windows CI cho fmt/check/clippy/test không cần phần cứng; kiểm tra thủ công refresh, cắm/rút và thoát app.
- [ ] **G1-09:** Đo baseline release build, cập nhật bằng chứng và báo cáo kết thúc giai đoạn.

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

Khi cập nhật nhật ký, dẫn tới task/PR/commit hoặc bản ghi kiểm chứng nếu có. Không chép log chứa dữ liệu cá nhân vào tài liệu.
