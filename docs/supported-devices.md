# Thiết bị và mức tương thích

**Hiện chưa có model nào được triển khai hoặc kiểm chứng bởi OpenGearHub.** Thư mục model/protocol đang là khung tài liệu.

Không suy ra hỗ trợ cả hãng từ một model. USB, Bluetooth và receiver phải có bằng chứng riêng khi hành vi khác nhau.

## Cách ghi nhận

| Mức | Ý nghĩa |
| --- | --- |
| Inventory observed | App đã phát hiện descriptor trên máy thật; chưa chứng minh đọc trạng thái hoặc điều khiển |
| Experimental | Đã có implementation/fixture, còn thiếu kiểm chứng phần cứng hoặc có giới hạn được ghi rõ |
| Hardware verified | Capability cụ thể đã kiểm thử trên đúng model/connection với kết quả và môi trường được ghi lại |

Mức được ghi theo capability. Ví dụ, xác minh đọc pin không đồng nghĩa xác minh RGB hoặc DPI.

## Bảng tương thích

| Model | Connection | Firmware | Capability | Mức xác minh | Windows/app revision | Ngày và bằng chứng | Giới hạn |
| --- | --- | --- | --- | --- | --- | --- | --- |

Thêm dòng thật khi có dữ liệu. Không điền model minh họa hoặc đánh dấu hỗ trợ từ tài liệu của dự án khác.

## Khi cập nhật

- Ghi đầy đủ model/connection, capability đã thử và kết quả thực.
- Firmware chưa biết ghi “Chưa biết”; không tự mở rộng kết luận sang firmware khác.
- Dẫn tới issue, PR hoặc bản ghi kiểm chứng đã làm sạch; không đăng serial hay HID dump chứa dữ liệu cá nhân.
- Lỗi mới hoặc thay đổi hành vi phải được ghi vào giới hạn và rà lại mức xác minh.
- Quy trình triển khai: [adding-device](adding-device.md). Checklist: [verification](verification.md).
