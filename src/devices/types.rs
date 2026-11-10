use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Bus {
    Usb,
    Bluetooth,
    I2c,
    Spi,
    Unknown,
}

impl Bus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Usb => "USB",
            Self::Bluetooth => "Bluetooth",
            Self::I2c => "I2C",
            Self::Spi => "SPI",
            Self::Unknown => "Chưa biết",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub path: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: Option<i32>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub release_number: u16,
    pub bus: Bus,
    pub container_id: Option<[u8; 16]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GroupId {
    Container([u8; 16]),
    Interface(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceGroup {
    pub id: GroupId,
    pub interfaces: Vec<Endpoint>,
}

pub fn group_endpoints(endpoints: Vec<Endpoint>) -> Vec<DeviceGroup> {
    let mut groups: BTreeMap<GroupId, Vec<Endpoint>> = BTreeMap::new();
    for endpoint in endpoints {
        let id = endpoint
            .container_id
            .filter(|id| *id != [0; 16])
            .map_or_else(
                || GroupId::Interface(endpoint.path.clone()),
                GroupId::Container,
            );
        groups.entry(id).or_default().push(endpoint);
    }
    groups
        .into_iter()
        .map(|(id, mut interfaces)| {
            interfaces.sort_by(|left, right| {
                (
                    &left.path,
                    left.usage_page,
                    left.usage,
                    left.interface_number,
                )
                    .cmp(&(
                        &right.path,
                        right.usage_page,
                        right.usage,
                        right.interface_number,
                    ))
            });
            DeviceGroup { id, interfaces }
        })
        .collect()
}

#[cfg(test)]
pub fn synthetic_endpoint(path: &str) -> Endpoint {
    Endpoint {
        path: path.to_owned(),
        vendor_id: 0xffff,
        product_id: 0x0001,
        usage_page: 0xff00,
        usage: 1,
        interface_number: Some(0),
        manufacturer: None,
        product: None,
        release_number: 0,
        bus: Bus::Usb,
        container_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_inventory_has_no_groups() {
        assert!(group_endpoints(Vec::new()).is_empty());
    }

    #[test]
    fn known_container_groups_interfaces_without_claiming_logical_device_count() {
        let mut first = synthetic_endpoint("synthetic://receiver/interface-1");
        first.container_id = Some([1; 16]);
        let mut second = synthetic_endpoint("synthetic://receiver/interface-0");
        second.container_id = first.container_id;
        let groups = group_endpoints(vec![first, second]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].id, GroupId::Container([1; 16]));
        assert_eq!(groups[0].interfaces.len(), 2);
        assert_eq!(
            groups[0].interfaces[0].path,
            "synthetic://receiver/interface-0"
        );
    }

    #[test]
    fn identical_model_ids_keep_distinct_interface_identities() {
        let first = synthetic_endpoint("synthetic://device-a/interface-0");
        let second = synthetic_endpoint("synthetic://device-b/interface-0");
        let groups = group_endpoints(vec![first, second]);
        assert_eq!(groups.len(), 2);
        assert_ne!(groups[0].id, groups[1].id);
        assert_eq!(groups[0].interfaces.len(), 1);
        assert_eq!(groups[1].interfaces.len(), 1);
    }

    #[test]
    fn distinct_known_containers_keep_identical_models_separate() {
        let mut first = synthetic_endpoint("synthetic://device-a/interface-0");
        first.container_id = Some([1; 16]);
        let mut second = synthetic_endpoint("synthetic://device-b/interface-0");
        second.container_id = Some([2; 16]);
        let groups = group_endpoints(vec![first, second]);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].id, GroupId::Container([1; 16]));
        assert_eq!(groups[1].id, GroupId::Container([2; 16]));
    }

    #[test]
    fn null_container_does_not_group_unrelated_interfaces() {
        let mut first = synthetic_endpoint("synthetic://device-a");
        first.container_id = Some([0; 16]);
        let mut second = synthetic_endpoint("synthetic://device-b");
        second.container_id = Some([0; 16]);
        let groups = group_endpoints(vec![first, second]);
        assert_eq!(groups.len(), 2);
        assert!(
            groups
                .iter()
                .all(|group| matches!(group.id, GroupId::Interface(_)))
        );
    }

    #[test]
    fn grouping_is_independent_of_discovery_order() {
        let mut first = synthetic_endpoint("synthetic://interface-b");
        first.container_id = Some([1; 16]);
        let mut second = synthetic_endpoint("synthetic://interface-a");
        second.container_id = first.container_id;
        let third = synthetic_endpoint("synthetic://ungrouped");
        assert_eq!(
            group_endpoints(vec![first.clone(), second.clone(), third.clone()]),
            group_endpoints(vec![third, second, first])
        );
    }

    #[test]
    fn absent_metadata_is_preserved_without_inventing_values() {
        let groups = group_endpoints(vec![synthetic_endpoint("synthetic://unknown")]);
        assert_eq!(groups[0].interfaces[0].manufacturer, None);
        assert_eq!(groups[0].interfaces[0].product, None);
        assert_eq!(groups[0].interfaces[0].container_id, None);
    }
}
