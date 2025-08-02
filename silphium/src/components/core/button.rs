use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Button)]
pub fn button(
    children: Html,
    #[prop_or_default] class: Classes,
    #[prop_or_default] title: Option<AttrValue>,
    #[prop_or_default] onclick: Callback<()>,
) -> Html {
    let onclick = onclick.reform(|_| ());
    html! {
      <button {class} {title} {onclick}>
        {children}
      </button>
    }
}
