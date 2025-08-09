use implicit_clone::ImplicitClone;
use yew::prelude::*;

use crate::{hooks::ModelHandle, model::Unit};

#[derive(PartialEq, Clone, ImplicitClone, Default)]
pub struct UnitFilter {
    pub era: Option<AttrValue>,
    pub horde: Option<bool>,
    pub regional: Option<bool>,
}

impl UnitFilter {
    pub fn apply(&self, unit: &Unit) -> bool {
        (if let Some(ref era) = self.era {
            unit.eras.contains(era)
        } else {
            true
        }) && (if let Some(horde) = self.horde {
            unit.horde == horde
        } else {
            true
        }) && (if let Some(regional) = self.regional {
            unit.is_regional == regional
        } else {
            true
        })
    }
}

impl ModelHandle<UnitFilter> {
    pub fn era_handle(&self) -> ModelHandle<Option<AttrValue>> {
        let model = self.clone();
        self.map(|f| f.era.clone(), move |e| UnitFilter { era: e, ..*model })
    }

    pub fn horde_handle(&self) -> ModelHandle<Option<bool>> {
        let model = self.clone();
        self.map(
            |f| f.horde,
            move |h| UnitFilter {
                horde: h,
                ..(*model).clone()
            },
        )
    }

    pub fn regional_handle(&self) -> ModelHandle<Option<bool>> {
        let model = self.clone();
        self.map(
            |f| f.regional,
            move |h| UnitFilter {
                regional: h,
                ..(*model).clone()
            },
        )
    }
}
