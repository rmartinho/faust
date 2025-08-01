use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Icon)]
pub fn icon(
    src: AttrValue,
    #[prop_or_default] class: Classes,
    #[prop_or_default] symbol: Option<AttrValue>,
    #[prop_or_default] title: Option<AttrValue>,
    #[prop_or_default] height: Option<usize>,
    #[prop_or_default] width: Option<usize>,
) -> Html {
    let image_class = match title {
        Some(_) => classes!(),
        _ => class.clone(),
    };
    let image = if let Some(symbol) = symbol {
        svg_symbol(src, symbol, image_class, height, width)
    } else {
        img(src, image_class, height, width)
    };

    if let Some(title) = title {
        html! {
          <div class={class} {title}>
            { image }
          </div>
        }
    } else {
        image
    }
}

fn svg_symbol(
    src: AttrValue,
    symbol: AttrValue,
    class: Classes,
    height: Option<usize>,
    width: Option<usize>,
) -> Html {
    let height = height
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "100%".into());
    let width = width
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "100%".into());
    html! {
      <svg role="img" {height} {width} {class}>
        <use href={format!("{src}#{symbol}")} />
      </svg>
    }
}

fn img(src: AttrValue, class: Classes, height: Option<usize>, width: Option<usize>) -> Html {
    let height: Option<AttrValue> = height.map(|h| format!("{}", h).into());
    let width: Option<AttrValue> = width.map(|w| format!("{}", w).into());
    html! {
      <img {src} {height} {width} {class} />
    }
}
