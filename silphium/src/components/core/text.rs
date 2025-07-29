use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Text)]
pub fn text(text: IString) -> Html {
    let lines = text
        .split("\n")
        .map(|s| html! { {s} })
        .intersperse(html! {<br/>});
    html! {
      {for lines}
    }
}
