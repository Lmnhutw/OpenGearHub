# Giao diện Slint

Chưa có file `.slint` hoặc giao diện chạy được. `main.slint` sẽ được thêm ở giai đoạn 1 cùng build integration của Rust.

- UI hiển thị model dữ liệu từ Rust và chuyển tiếp thao tác qua callback.
- Bản đầu có danh sách–chi tiết, refresh, loading/empty/error/unsupported và dữ liệu chưa biết.
- Không chứa VID/PID, encode report hoặc nhánh theo hãng/model.
- Control dùng chung được đặt trong `controls` khi có capability thực cần hiển thị.
- Kiểm tra render, resize, focus và thao tác bàn phím theo [verification](../docs/verification.md).

Đối chiếu tài liệu Slint đúng phiên bản đã chọn; phiên bản trong kế hoạch chưa phải dependency đã cài.
