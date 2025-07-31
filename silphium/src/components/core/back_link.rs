use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use crate::components::Link;
use crate::routes::Route;

#[autoprops]
#[function_component(BackLink)]
pub fn back_link(#[prop_or_default] title: Option<IString>) -> Html {
    let route = use_route::<Route>().unwrap_or_default().back();

    html! {
      <Link to={route}>
        <img class="back" {title} src="/icons/ui/back.webp" />
      </Link>
    }
}
