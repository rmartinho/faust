use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    AppContext,
    components::{BackLink, Button, Dialog, HelpDialog, MercenaryRoster, RosterFilter, UnitFilter},
    hooks::ModelHandle,
    model::Module,
};

#[autoprops]
#[function_component(MercenaryPage)]
pub fn mercenary_page(module_id: AttrValue) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = &ctx.modules[&module_id];

    let filter = use_state(|| UnitFilter::default());

    // TODO refactor this out in common with FactionPage
    let help_dialog = use_state(|| None as Option<Box<dyn Dialog>>);

    let show_help = Callback::from({
        let help_dialog = help_dialog.clone();
        move |()| {
            help_dialog.as_ref().unwrap().show();
        }
    });

    html! {
    <div class="faction-page">
      <header class="header-container">
        <div class="nav">
          <BackLink />
          // <Button>
          //   <img class="settings button" title="Configure" src="/icons/ui/settings.webp" />
          // </Button>
          <Button onclick={show_help}>
            <img class="help button" title="Help" src="/icons/ui/help.webp" />
          </Button>
          <HelpDialog control={help_dialog.setter().to_callback()} />
        </div>
        <MercenaryHeader class="header" {module} filter={&filter} />
      </header>
      <main>
        <MercenaryRoster pools={&module.pools} filter={&*filter} />
      </main>
    </div>
    }
}

#[autoprops]
#[function_component(MercenaryHeader)]
fn mercenary_header(class: Classes, module: Module, filter: ModelHandle<UnitFilter>) -> Html {
    html! {
      <div class={classes!("faction-header", class)}>
        <div class="title">
          <div class="name">{"Mercenaries"}</div>
          <RosterFilter {module} {filter} />
        </div>
        <img class="icon" src="/icons/ui/mercs.webp" />
      </div>
    }
}
