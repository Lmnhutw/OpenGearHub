# Third-party notices

Source OpenGearHub giữ [MIT](LICENSE). Tài liệu này ghi lựa chọn license cho các dependency trực tiếp của Phase 1; không thay thế license của từng package.

## Dependency trực tiếp

| Dependency | Phiên bản/nhánh | Lựa chọn đang dùng | Nguồn |
| --- | --- | --- | --- |
| Slint, slint-build và các thành phần framework Slint tương ứng | 1.18.1 | Slint Royalty-free Desktop, Mobile, and Web Applications License 2.0 | [Upstream v1.18.1](https://github.com/slint-ui/slint/blob/v1.18.1/LICENSE.md) |
| hidapi Rust wrapper | 2.6.7, feature windows-native | MIT | [hidapi-rs](https://github.com/ruabmbua/hidapi-rs) |
| windows-sys | 0.61, bản cụ thể theo Cargo.lock | MIT trong lựa chọn MIT OR Apache-2.0 | [windows-rs](https://github.com/microsoft/windows-rs) |

Các bản license trực tiếp được giữ tại:

- [Slint Royalty-free 2.0](licenses/Slint-Royalty-free-2.0.md)
- [hidapi-rs MIT](licenses/hidapi-rs-MIT.txt)
- [windows-sys MIT](licenses/windows-sys-MIT.txt)

Các file trên được sao chép nguyên văn từ package đã tải trong Cargo registry. Lockfile xác định phiên bản package và checksum dùng cho build.

## Attribution Slint

App chọn Royalty-free 2.0 và cung cấp `AboutSlint` trong **Trợ giúp → Giới thiệu OpenGearHub**, truy cập từ menu cấp cao nhất. Giữ attribution này trong binary phân phối. Việc source app dùng MIT không thay đổi điều khoản của Slint.

License Royalty-free này áp dụng cho ứng dụng desktop/mobile/web trong phạm vi điều khoản của nó; không mở rộng sang embedded, phân phối Slint độc lập hoặc công khai API Slint. Xem toàn văn đã lưu thay vì suy ra quyền dùng từ tên license.

## Trước khi phân phối binary

**Chưa hoàn tất audit license của toàn bộ dependency bắc cầu hoặc asset.** Các notice trực tiếp ở trên chưa phải gói notice đầy đủ cho mọi binary.

- Rà dependency graph theo Cargo.lock và features/target thực tế của binary release.
- Gom license/copyright/notice cần thiết của dependency bắc cầu, compiler output có yêu cầu liên quan, font và asset đóng gói.
- Giữ license/notice nguyên bản khi tái sử dụng source; kiểm tra attribution Slint trong binary thực.
- Ghi kết quả audit và cung cấp notice cùng gói phát hành trước khi công bố hoàn tất rà soát phân phối.

hidapi hiện dùng Windows native backend; không coi các lựa chọn license của C HIDAPI vendored trong source package là đã tự động áp dụng cho mọi backend hoặc build khác.
