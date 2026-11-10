# Fixtures

Chỗ dành cho descriptor/report phục vụ test khi logic tương ứng được triển khai. Hiện chưa có fixture hoặc test.

Mỗi fixture cần ghi nguồn gốc trong README hoặc metadata đi kèm:

- Task/model/protocol/connection liên quan và format dữ liệu.
- `synthetic` hoặc `captured`; với capture ghi môi trường/firmware nếu biết.
- Những trường đã làm sạch và ảnh hưởng của việc làm sạch tới test.
- Trường hợp mong đợi: hợp lệ, thiếu metadata, sai độ dài, match mơ hồ hoặc lỗi khác.

Không đưa serial, đường dẫn cá nhân, phím gõ hoặc USB capture thừa dữ liệu vào repo. Fixture giả lập không phải bằng chứng hỗ trợ phần cứng.

Xem [verification](../../docs/verification.md) và [adding-device](../../docs/adding-device.md).
