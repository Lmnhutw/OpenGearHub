# Giao diện Slint

`main.slint` khai báo AppWindow và được compile qua build.rs bằng Slint 1.18.1. UI dùng Winit, software renderer và accessibility.

- UI hiển thị model dữ liệu từ Rust và chuyển tiếp thao tác qua callback.
- Có danh sách–chi tiết, lựa chọn bằng bàn phím, refresh qua button/menu/F5 và loading/empty/error/unsupported. Refresh lỗi giữ dữ liệu trước với thông báo chưa cập nhật.
- Hàng được ghi rõ Nhóm Windows hoặc Interface chưa ghép; không tuyên bố số nhóm là số thiết bị ngoại vi logic.
- Không chứa VID/PID, encode report hoặc nhánh theo hãng/model.
- Control dùng chung được đặt trong `controls` khi có capability thực cần hiển thị.
- Menu Trợ giúp → Giới thiệu OpenGearHub chứa AboutSlint cho lựa chọn license Royalty-free 2.0.
- Kiểm tra render, resize, focus và thao tác bàn phím theo [verification](../docs/verification.md).

Chưa có control pin/DPI/RGB hoặc chức năng điều khiển phần cứng. Kết quả render/tương tác thực tế được ghi trong tài liệu kiểm chứng, không suy ra từ việc compile thành công.
