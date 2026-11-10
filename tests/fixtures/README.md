# Fixtures

Các test unit hiện dùng descriptor synthetic trực tiếp trong module Rust: path `synthetic://...`, VID `0xffff`, PID `0x0001`, usage page `0xff00`. Đây là dữ liệu tự tạo để kiểm tra grouping, metadata thiếu và matcher; không được capture từ thiết bị thật và không chứng minh model nào được hỗ trợ.

Hiện chưa có file HID report hoặc capture phần cứng. Khi cần dữ liệu dạng file, đặt tại thư mục này và ghi nguồn gốc theo hướng dẫn dưới đây.

Mỗi fixture cần ghi nguồn gốc trong README hoặc metadata đi kèm:

- Task/model/protocol/connection liên quan và format dữ liệu.
- `synthetic` hoặc `captured`; với capture ghi môi trường/firmware nếu biết.
- Những trường đã làm sạch và ảnh hưởng của việc làm sạch tới test.
- Trường hợp mong đợi: hợp lệ, thiếu metadata, sai độ dài, match mơ hồ hoặc lỗi khác.

Không đưa serial, đường dẫn cá nhân, phím gõ hoặc USB capture thừa dữ liệu vào repo. Fixture giả lập không phải bằng chứng hỗ trợ phần cứng.

Xem [verification](../../docs/verification.md) và [adding-device](../../docs/adding-device.md).
