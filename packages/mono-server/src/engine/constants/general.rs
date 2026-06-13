use std::sync::LazyLock;

use ashscript_types::components::body::{UnitBody, UnitPart};

pub static STARTING_UNIT_BODY: LazyLock<UnitBody> = LazyLock::new(|| {
    UnitBody::from_vec(vec![
        (UnitPart::Generate, 15),
        (UnitPart::Fabricate, 3),
        (UnitPart::Convert, 5),
        (UnitPart::Ranged, 6),
        (UnitPart::Shield, 3),
        (UnitPart::RangeImprovement, 4),
        (UnitPart::DamageImprovement, 2),
    ])
});