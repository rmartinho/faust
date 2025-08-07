use web_sys::HtmlElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_hooks::prelude::*;

use crate::{components::UnitCard, model::Unit};

const HELP_UNIT_CBOR: &[u8] = include_bytes!("help-unit.cbor");

pub trait Dialog {
    fn show(&self);
    #[allow(dead_code)]
    fn hide(&self);
}

struct PopOver {
    el: HtmlElement,
}

impl Dialog for PopOver {
    fn show(&self) {
        let _ = self.el.show_popover();
    }

    fn hide(&self) {
        let _ = self.el.hide_popover();
    }
}

#[autoprops]
#[function_component(HelpDialog)]
pub fn help_dialog(#[prop_or_default] control: Callback<Option<Box<dyn Dialog>>>) -> Html {
    let popover_ref = use_node_ref();

    use_effect({
        let popover_ref = popover_ref.clone();
        move || {
            let popover = popover_ref.cast::<HtmlElement>().unwrap();
            control.emit(Some(Box::new(PopOver { el: popover })));
        }
    });

    let hide = {
        let popover_ref = popover_ref.clone();
        move |_| {
            let popover = popover_ref.cast::<HtmlElement>().unwrap();
            let _ = popover.hide_popover();
        }
    };
    use_click_away(popover_ref.clone(), hide.clone());
    // let hide = Callback::from(move |e| hide(Event::from(e)));

    let unit: Unit = ciborium::from_reader(HELP_UNIT_CBOR).unwrap();
    html! {
      <div ref={popover_ref} popover="">
        <div  class="help-dialog">
          <div class="descr left">
            <div class="name">{"Name"}</div>
            <div class="soldiers">{"Formation"}</div>
            <div class="cost">{"Recruitment"}</div>
            <div class="upkeep">{"Upkeep"}</div>
            <div class="mental">{"Soldiers"}</div>
            <div class="terrain">{"Terrain"}</div>
            <div class="weapons">{"Weapons"}</div>
            <div class="defenses">{"Defense"}</div>
          </div>
          <UnitCard {unit} />
          <div class="descr right">
            <div class="abilities">{"Abilities"}</div>
          </div>
        </div>
      </div>
    }
}
