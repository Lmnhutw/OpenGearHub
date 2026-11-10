# Hướng dẫn thêm thiết bị

Quy trình này áp dụng từ giai đoạn 2. Hiện chưa có driver thật. Đọc [kiến trúc](architecture.md), [scope](../PROJECT.md) và [bảng tương thích](supported-devices.md) trước khi thêm model.

## 1. Thu thập dữ kiện

- Model chính xác, hãng, VID/PID, kiểu kết nối USB/Bluetooth/receiver và firmware nếu biết.
- Interface/usage page/usage cần thiết; receiver có thể quản lý nhiều thiết bị logic.
- Thiết bị thật và Windows dùng để kiểm thử. Lưu serial/đường dẫn riêng tư ngoài fixture công khai.
- Nguồn mô tả giao thức, phiên bản nguồn và license nếu tái sử dụng code.

Discovery chỉ chứng minh nhìn thấy thiết bị. Không coi VID/PID hoặc tên hãng là bằng chứng dùng được mọi lệnh.

## 2. Chọn phạm vi thay đổi

| Trường hợp | Cách triển khai |
| --- | --- |
| Protocol và capability đã có, hành vi tương thích | Thêm model specification và giới hạn; đăng ký mapping nếu cần |
| Protocol mới | Thêm bộ xử lý trong `src/devices/protocols`, nối registry/factory |
| Capability mới | Thêm command/state có kiểu và control UI dùng chung cho capability |
| Transport/API mới | Thêm adapter phù hợp và kiểm thử quyền truy cập/vòng đời riêng |

Tên thư mục hoặc hãng mới không phải lý do để sao chép implementation protocol. Chỉ chia sẻ code khi đã xác minh tương thích.

## 3. Thêm mapping nhận diện

- Đặt dữ liệu model trong `src/devices/models`; khai báo protocol, connection và các giới hạn đã biết.
- Match đủ điều kiện interface/firmware có ảnh hưởng. Thêm test cho match đúng, không match và match mơ hồ.
- Giữ hai thiết bị cùng VID/PID là hai identity độc lập. Không dùng chỉ số hàng UI làm đích lệnh.
- Không match hoặc chưa đủ bằng chứng thì giữ trạng thái chưa hỗ trợ; không gửi report dò tùy ý.

## 4. Đọc trước, điều khiển sau

- Triển khai decode report và đọc trạng thái; xử lý dữ liệu thiếu, report sai độ dài và phản hồi chưa biết.
- Khi thêm lệnh ghi, validate capability, khoảng giá trị, format/report ID và đúng phiên kết nối.
- Ghi cách xác nhận: phản hồi, readback hoặc tiêu chí do giao thức quy định. Nếu không xác nhận được thì trạng thái phải thể hiện giới hạn đó.
- Ưu tiên tính năng có thể khôi phục; ghi rõ thao tác tạm thời hay lưu vào bộ nhớ thiết bị. Không flash firmware trong scope hiện tại.
- Kiểm tra timeout của API thực tế; không retry mù lệnh ghi.

## 5. Test và bằng chứng

- Unit test encode/decode, report sai, validation, capability và matcher liên quan.
- Fixture đặt tại `tests/fixtures`, ghi model/protocol, connection, nguồn gốc, thay đổi làm sạch và trạng thái synthetic/captured.
- Kiểm thử phần cứng theo [verification](verification.md): đọc/ghi, disconnect/reconnect, xung đột app hãng và tác động lên thiết bị khác.
- Dữ liệu synthetic chỉ kiểm chứng logic; không chứng minh hỗ trợ phần cứng.

## 6. Hoàn tất thay đổi

- [ ] Cập nhật mapping/protocol và test liên quan.
- [ ] UI/worker chung không cần thay đổi cho model dùng capability hiện có.
- [ ] Ghi mức xác minh theo model, connection và capability trong bảng tương thích.
- [ ] Chạy các check phù hợp, ghi phần chưa chạy và giới hạn.
- [ ] Cập nhật nhiệm vụ/nhật ký trong [roadmap](roadmap.md).
- [ ] Giữ notice/license của code tham khảo hoặc tái sử dụng.

Không thêm model vào danh sách “đã kiểm thử phần cứng” khi chỉ có enumeration, fixture giả lập hoặc CI xanh.
