use web_sys::HtmlImageElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_hooks::prelude::*;

use crate::{
    components::{Icon, OptionButton, OptionGroup, Text, ToggleButton, UnitFilter},
    hooks::ModelHandle,
    model::{Faction, Module},
};

#[autoprops]
#[function_component(RosterFilter)]
pub fn roster_filter(
    module: Module,
    #[prop_or_default] faction: Option<Faction>,
    filter: ModelHandle<UnitFilter>,
) -> Html {
    let horde = filter.horde_handle();
    let horde = (*horde).map(|_| horde.map(|h| h.unwrap(), Option::Some));

    let era = filter.era_handle();
    let era = era
        .as_ref()
        .map(|_| era.map(|e| e.clone().unwrap(), Option::Some));

    use_effect_once({
        let m_eras = module.eras.clone();
        let f_eras = faction
            .as_ref()
            .map_or(Default::default(), |f| f.eras.clone());
        move || {
            f_eras.into_iter().for_each(|e| {
                let info = &m_eras[e];
                preload_image(&info.icon);
                preload_image(&info.icoff);
            });
            || {}
        }
    });

    let era_options = faction
        .map_or(Default::default(), |f| f.eras)
        .iter()
        .map(move |e| {
            let info = &module.eras[&e];
            let active = filter.era == Some(e.clone());
            html_nested! {
              <OptionButton value={e} class={classes!("era", active.then_some("checked"))}>
                <img src={if active { &info.icon } else { &info.icoff }} title={&info.name} />
                <span><Text text={&info.name} /></span>
              </OptionButton>
            }
        });

    html! {
      <>
        if let Some(era) = era {
          <OptionGroup class="eras" name="era" value={era}>
            {for era_options}
          </OptionGroup>
        }
        if let Some(horde) = horde {
          <div class="eras">
            <ToggleButton value={&horde}
                class={classes!("era", horde.then_some("checked"))}
                title={if *horde { "Show settled units" } else { "Show horde units" }}
            >
              <Icon src="/icons/ui/horde.svg" symbol={if *horde {"on"} else {"off"}} />
            </ToggleButton>
          </div>
        }
      </>
    }
}

fn preload_image(src: &str) {
    let image = HtmlImageElement::new().unwrap();
    image.set_src(src);
}
