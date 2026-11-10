# Khung source Rust

Hiện chưa có Cargo project hoặc file Rust. Cấu trúc đích được mô tả trong [kiến trúc](../docs/architecture.md); công việc bắt đầu tại G1-01 của [roadmap](../docs/roadmap.md).

| File/module dự kiến | Trách nhiệm |
| --- | --- |
| `main.rs` | Khởi tạo, wiring và vòng đời app |
| `app.rs` | App state và callback UI |
| `devices/types.rs` | Descriptor, identity, capability và snapshot |
| `devices/discovery.rs` | Inventory và chuẩn hóa metadata |
| `devices/registry.rs` | Mapping model/interface tới bộ xử lý |
| `devices/worker.rs` | Hàng đợi, context/handle và I/O |
| `devices/driver.rs` | Contract điều khiển khi có protocol thật ở giai đoạn 2 |

Chỉ tạo file khi có nội dung triển khai thực; không thêm struct/trait giả để lấp cấu trúc. Worker và UI không chứa điều kiện riêng theo hãng/model.
