# Báo cáo Phase 1 — HID inventory

Ngày: 2026-11-10. Phiên bản app: 0.1.0; thay đổi hiện trong working tree, chưa commit/push. Phase 1 đã triển khai và qua kiểm chứng chức năng bên dưới; Phase 2 chưa bắt đầu.

## 1. Đã triển khai

- Một Cargo package/executable Rust edition 2024, MSRV 1.92, Slint 1.18.1, Winit/software renderer/accessibility.
- Inventory HID chỉ đọc; normalize metadata; grouping theo Windows Container ID, giữ interface chưa nhóm riêng biệt.
- Registry production rỗng; matcher thuần với specification synthetic trong test.
- Một worker với command/result queue dung lượng 1, gửi không chặn, bỏ refresh lặp và không I/O trên UI thread.
- Refresh lỗi giữ snapshot/lựa chọn cũ; lỗi trước snapshot đầu tiên không bị trình bày như inventory rỗng thành công. Refresh thành công giữ lựa chọn theo identity hoặc xóa khi thiết bị biến mất.
- UI tiếng Việt danh sách–chi tiết, focus bàn phím, F5, loading/empty/error/unsupported; raw path/serial không được hiển thị.
- Trợ giúp → Giới thiệu có AboutSlint; source MIT giữ nguyên, license Slint được chọn là Royalty-free 2.0. Xem [notices](../THIRD_PARTY_NOTICES.md).
- Workflow Windows có fmt/check/clippy/test/release build bằng Rust 1.96 và MSRV check bằng 1.92.

## 2. Kết quả tự động

Môi trường local: Windows 10 Pro 22H2 build 19045.6456 x64; Rust/Cargo 1.96.0; Rust 1.92.0 được cài thêm để kiểm tra MSRV; MSVC 14.52.37025 trong Visual Studio Insiders; Windows SDK 10.0.26100.0.

| Command | Kết quả |
| --- | --- |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo check --locked --target x86_64-pc-windows-msvc` | Exit 0 |
| `cargo clippy --locked --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | Exit 0, không warning |
| `cargo test --locked --target x86_64-pc-windows-msvc` | Exit 0: **29 passed; 0 failed; 0 ignored** |
| `cargo build --release --locked --target x86_64-pc-windows-msvc` | Exit 0 |
| `cargo +1.92.0 check --locked --target x86_64-pc-windows-msvc --target-dir target/msrv` | Exit 0 |

Workflow đã tạo, chưa được chạy trên GitHub vì changeset chưa push. Không suy ra CI remote xanh từ kết quả local.

## 3. Đối chiếu yêu cầu với test

Tên dưới đây là tên test thực; dữ liệu test là synthetic, không chứng minh hỗ trợ điều khiển phần cứng.

| Yêu cầu | Test/bằng chứng |
| --- | --- |
| Nhiều interface cùng container; không coi receiver là một model điều khiển | `known_container_groups_interfaces_without_claiming_logical_device_count` |
| Hai thiết bị cùng VID/PID vẫn độc lập | `identical_model_ids_keep_distinct_interface_identities`, `distinct_known_containers_keep_identical_models_separate` |
| Container rỗng/không có không gộp nhầm; thứ tự ổn định; inventory rỗng | `null_container_does_not_group_unrelated_interfaces`, `grouping_is_independent_of_discovery_order`, `empty_inventory_has_no_groups` |
| Metadata thiếu và chuỗi rỗng | `missing_and_blank_metadata_remain_unknown`, `metadata_preserves_content_without_outer_padding`, `absent_metadata_is_preserved_without_inventing_values` |
| Matcher đúng và không match khi sai điều kiện/thiếu metadata bắt buộc | `exact_synthetic_match_returns_model_id`, `empty_group_does_not_match`, `wrong_vendor_product_usage_interface_or_bus_does_not_match`, `missing_required_interface_does_not_match`, `unconstrained_interface_can_match_missing_interface_metadata` |
| Match mơ hồ; nhiều interface không tạo match model trùng; registry production rỗng | `multiple_model_matches_are_ambiguous`, `multiple_matching_interfaces_do_not_make_one_model_ambiguous`, `production_registry_never_claims_untested_hardware_support` |
| Refresh lặp không xếp thêm scan; kết quả được tiêu thụ thì quét tiếp được | `repeated_refresh_does_not_queue_more_scans` |
| Discovery lỗi/panic trả kết quả lỗi | `discovery_error_and_panic_return_failures_instead_of_stuck_loading` |
| Đóng khi scan bị chặn không join vô hạn hoặc publish kết quả muộn | `closing_blocked_discovery_does_not_join_or_publish_late_result`, `closed_state_rejects_refresh_and_late_result` |
| Lỗi refresh giữ toàn bộ snapshot/lựa chọn; lỗi worker không giữ loading | `failed_refresh_preserves_stale_inventory_and_selection`, `unavailable_worker_ends_loading_with_visible_error` |
| Refresh phục hồi xóa lỗi; lựa chọn theo identity và xóa khi bị rút | `successful_refresh_clears_previous_error`, `successful_refresh_preserves_identity_and_clears_removed_selection` |
| Không lộ path; hiển thị metadata chưa biết/unsupported | `details_hide_interface_path_and_preserve_unknown_metadata` |
| FFI không cắt identity ở NUL và chấp nhận lookup không có kết quả | `embedded_nul_is_rejected_without_truncating_identity`, `nonexistent_interface_has_no_container_metadata` |
| UI thật: phím xuống chọn hàng và gọi host callback, F5 khi rảnh/bận/sau quét, resize 1100×720 và 800×520 | `ui_tests::app_window_handles_keyboard_selection_and_refresh_gating_at_both_sizes` |

Test UI dùng Slint platform test trong bộ nhớ; không đọc phần cứng và không tạo/chụp/lưu ảnh. Theo yêu cầu người dùng, ảnh bằng chứng không còn là điều kiện kiểm chứng mặc định.

## 4. Chạy thật và xác nhận của người dùng

- Bản release cuối khởi động và enumerate được **16 interface / 3 nhóm Windows**; cây accessibility thể hiện danh sách và chi tiết, including receiver descriptor VID:PID 046D:C52B.
- Log chỉ ghi thời gian và số lượng, không dump raw path/serial/report.
- Trong phiên kiểm thử, log ghi chuỗi **16 interface / 3 nhóm → 10 interface / 2 nhóm → 16 interface / 3 nhóm** sau các lượt refresh.
- Người dùng xác nhận đã hoàn tất chọn thiết bị, F5, cắm/rút USB rồi refresh và đóng app bằng nút X.
- Process kiểm thử kết thúc với **exit 0**, không còn process OpenGearHub sau thao tác đóng.

Công cụ điều khiển Windows đọc được accessibility nhưng capture/input tự động gặp lỗi môi trường (`window capture timed out`, `window is not a usable app window`). Kết quả thao tác desktop ở trên được ghi là **xác nhận của người dùng**, tách khỏi test headless và quan sát log; không công bố đã tự động kiểm tra thao tác native này.

## 5. Baseline release

CPU: Intel Core i7-13700, 24 logical processors. RAM máy: 34,033,348,608 bytes. Binary đo là bản release software renderer, SHA256:

```text
55B342A46BD5769605930A475E3194D34F782663D66B702FFD4272407590BE00
```

| Số liệu | Quan sát |
| --- | --- |
| Từ vào `main` đến `show()` trả về | 10 ms |
| Discovery đầu tiên | 13 ms, 16 interface / 3 nhóm |
| Discovery trong các lượt refresh quan sát | 10–17 ms; số interface phụ thuộc thiết bị đang cắm |
| Khoảng lấy mẫu CPU/bộ nhớ | 10.028 giây sau inventory đầu tiên |
| CPU tiến trình, chia cho 24 logical processors | 0.0454% trong mẫu trên |
| Private bytes | 8,896,512 bytes, khoảng 8.5 MiB |
| Working set | 25,321,472 bytes, khoảng 24.1 MiB |

CPU được tính bằng chênh lệch `TotalProcessorTime / elapsed / logical_processors`; bộ nhớ đọc từ process .NET sau khoảng lấy mẫu. Startup nội bộ dùng Rust `Instant`. Dữ liệu máy được đọc từ Windows CIM, không lưu hostname/user/serial.

Đây là một mẫu trong phiên kiểm thử có thể có thao tác người dùng và tác vụ nền. Không coi là benchmark cô lập hoặc cam kết budget. Chưa đo loader/process-start tới first paint, CPU peak khi refresh hoặc tray. `show()` trả về không đồng nghĩa đã đo first paint. Working set và private bytes là hai chỉ số khác nhau.

## 6. Giới hạn và bước tiếp theo

- Enumeration không có cancellation: đóng app ngừng nhận lệnh, bỏ kết quả muộn và detach worker chưa xong; không cam kết hủy lời gọi Windows hoặc chạy toàn bộ destructor trước khi process thoát.
- HID backend có thể bỏ qua interface không đọc metadata được. Container ID là nhóm Windows, không chứng nhận thiết bị logic hay giao thức.
- Chưa có model điều khiển, protocol hãng, pin/DPI/RGB/audio/profile/tray. Giai đoạn 2 cần model/kết nối/giao thức và thiết bị kiểm thử cụ thể.
- Trước khi phát hành binary, còn audit license/notice đầy đủ của dependency bắc cầu và asset.
- Chưa commit/push thay đổi; CI remote và matrix Windows 11 chưa được chạy.

Tiến độ chính thức ở [roadmap](roadmap.md). Dừng ở Phase 1, chưa tự chuyển Phase 2.
