use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::hooks::ModelHandle;

#[autoprops]
#[function_component(ToggleButton)]
pub fn toggle_button(
    children: Html,
    #[prop_or_default] class: Classes,
    #[prop_or_default] title: Option<AttrValue>,
    value: ModelHandle<bool>,
) -> Html {
    let onchange = value.reduce_callback(|b| !*b);
    html! {
      <label {class} {title}>
        <input type="checkbox" checked={*value} {onchange} style="display:none" />
        {children}
      </label>
    }
}
