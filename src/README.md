# Source Rust

Phase 1 đã có Cargo project và các module inventory HID chỉ đọc. Cấu trúc và ranh giới được mô tả trong [kiến trúc](../docs/architecture.md); kết quả kiểm chứng nằm trong [roadmap](../docs/roadmap.md).

| File/module | Trách nhiệm |
| --- | --- |
| `main.rs` | Khởi tạo, wiring và vòng đời app |
| `app.rs` | App state và callback UI |
| `devices/types.rs` | Descriptor, identity, grouping interface và bus |
| `devices/discovery.rs` | Inventory và chuẩn hóa metadata |
| `devices/registry.rs` | Matcher và registry; hiện chưa đăng ký model điều khiển |
| `devices/worker.rs` | Hàng đợi, context/handle và I/O |
| `devices/driver.rs` (chưa tạo) | Contract điều khiển khi có protocol thật ở giai đoạn 2 |

Worker dùng hàng đợi có giới hạn, bỏ refresh lặp khi còn pending và chuyển kết quả về UI bằng event loop. Lỗi scan giữ inventory trước; shutdown ngừng nhận lệnh và bỏ kết quả muộn. Enumeration không hủy được trong backend hiện tại nên worker đang bận được detach khi thoát.

Model, protocol và capability điều khiển vẫn thuộc giai đoạn 2. Worker và UI không chứa điều kiện riêng theo hãng/model.
