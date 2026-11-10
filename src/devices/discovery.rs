use hidapi::{BusType, DeviceInfo, HidApi};

use super::types::{Bus, DeviceGroup, Endpoint, group_endpoints};
use crate::platform;

pub fn scan() -> Result<Vec<DeviceGroup>, String> {
    let api = HidApi::new()
        .map_err(|_| "Không thể quét HID trên Windows. Hãy thử làm mới.".to_owned())?;
    let endpoints = api
        .device_list()
        .map(endpoint)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(group_endpoints(endpoints))
}

fn endpoint(info: &DeviceInfo) -> Result<Endpoint, String> {
    let path = info
        .path()
        .to_str()
        .map_err(|_| {
            "Một interface HID có identity không đọc được. Inventory chưa được cập nhật.".to_owned()
        })?
        .to_owned();
    let container_id = platform::container_id(&path);
    Ok(Endpoint {
        path,
        vendor_id: info.vendor_id(),
        product_id: info.product_id(),
        usage_page: info.usage_page(),
        usage: info.usage(),
        interface_number: (info.interface_number() >= 0).then(|| info.interface_number()),
        manufacturer: normalize_text(info.manufacturer_string()),
        product: normalize_text(info.product_string()),
        release_number: info.release_number(),
        bus: match info.bus_type() {
            BusType::Usb => Bus::Usb,
            BusType::Bluetooth => Bus::Bluetooth,
            BusType::I2c => Bus::I2c,
            BusType::Spi => Bus::Spi,
            BusType::Unknown => Bus::Unknown,
        },
        container_id,
    })
}

fn normalize_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::normalize_text;

    #[test]
    fn missing_and_blank_metadata_remain_unknown() {
        assert_eq!(normalize_text(None), None);
        assert_eq!(normalize_text(Some("")), None);
        assert_eq!(normalize_text(Some(" \t\r\n")), None);
    }

    #[test]
    fn metadata_preserves_content_without_outer_padding() {
        assert_eq!(
            normalize_text(Some("  Synthetic Device  ")),
            Some("Synthetic Device".to_owned())
        );
    }
}
