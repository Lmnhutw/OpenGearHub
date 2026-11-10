use super::types::{Bus, DeviceGroup, Endpoint};

struct ModelSpec {
    id: &'static str,
    vendor_id: u16,
    product_id: u16,
    usage_page: u16,
    usage: u16,
    interface_number: Option<i32>,
    bus: Bus,
}

impl ModelSpec {
    fn matches(&self, endpoint: &Endpoint) -> bool {
        self.vendor_id == endpoint.vendor_id
            && self.product_id == endpoint.product_id
            && self.usage_page == endpoint.usage_page
            && self.usage == endpoint.usage
            && self
                .interface_number
                .is_none_or(|number| endpoint.interface_number == Some(number))
            && self.bus == endpoint.bus
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchResult {
    Unsupported,
    Matched(&'static str),
    Ambiguous,
}

const MODELS: &[ModelSpec] = &[];

pub fn resolve(group: &DeviceGroup) -> MatchResult {
    resolve_with(group, MODELS)
}

fn resolve_with(group: &DeviceGroup, models: &[ModelSpec]) -> MatchResult {
    let mut matching = models.iter().filter(|model| {
        group
            .interfaces
            .iter()
            .any(|endpoint| model.matches(endpoint))
    });
    match (matching.next(), matching.next()) {
        (None, _) => MatchResult::Unsupported,
        (Some(model), None) => MatchResult::Matched(model.id),
        (Some(_), Some(_)) => MatchResult::Ambiguous,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::types::{GroupId, synthetic_endpoint};

    fn group() -> DeviceGroup {
        DeviceGroup {
            id: GroupId::Interface("synthetic://interface-0".to_owned()),
            interfaces: vec![synthetic_endpoint("synthetic://interface-0")],
        }
    }

    fn spec(id: &'static str) -> ModelSpec {
        ModelSpec {
            id,
            vendor_id: 0xffff,
            product_id: 1,
            usage_page: 0xff00,
            usage: 1,
            interface_number: Some(0),
            bus: Bus::Usb,
        }
    }

    #[test]
    fn production_registry_never_claims_untested_hardware_support() {
        assert_eq!(resolve(&group()), MatchResult::Unsupported);
    }

    #[test]
    fn exact_synthetic_match_returns_model_id() {
        assert_eq!(
            resolve_with(&group(), &[spec("synthetic-a")]),
            MatchResult::Matched("synthetic-a")
        );
    }

    #[test]
    fn empty_group_does_not_match() {
        let mut group = group();
        group.interfaces.clear();
        assert_eq!(
            resolve_with(&group, &[spec("synthetic-a")]),
            MatchResult::Unsupported
        );
    }

    #[test]
    fn wrong_vendor_product_usage_interface_or_bus_does_not_match() {
        let mutations: [fn(&mut Endpoint); 6] = [
            |endpoint| endpoint.vendor_id = 1,
            |endpoint| endpoint.product_id = 2,
            |endpoint| endpoint.usage_page = 1,
            |endpoint| endpoint.usage = 2,
            |endpoint| endpoint.interface_number = Some(1),
            |endpoint| endpoint.bus = Bus::Bluetooth,
        ];
        for mutate in mutations {
            let mut group = group();
            mutate(&mut group.interfaces[0]);
            assert_eq!(
                resolve_with(&group, &[spec("synthetic-a")]),
                MatchResult::Unsupported
            );
        }
    }

    #[test]
    fn missing_required_interface_does_not_match() {
        let mut group = group();
        group.interfaces[0].interface_number = None;
        assert_eq!(
            resolve_with(&group, &[spec("synthetic-a")]),
            MatchResult::Unsupported
        );
    }

    #[test]
    fn unconstrained_interface_can_match_missing_interface_metadata() {
        let mut group = group();
        group.interfaces[0].interface_number = None;
        let mut model = spec("synthetic-a");
        model.interface_number = None;
        assert_eq!(
            resolve_with(&group, &[model]),
            MatchResult::Matched("synthetic-a")
        );
    }

    #[test]
    fn multiple_model_matches_are_ambiguous() {
        assert_eq!(
            resolve_with(&group(), &[spec("synthetic-a"), spec("synthetic-b")]),
            MatchResult::Ambiguous
        );
    }

    #[test]
    fn multiple_matching_interfaces_do_not_make_one_model_ambiguous() {
        let mut group = group();
        group
            .interfaces
            .push(synthetic_endpoint("synthetic://interface-1"));
        assert_eq!(
            resolve_with(&group, &[spec("synthetic-a")]),
            MatchResult::Matched("synthetic-a")
        );
    }
}
