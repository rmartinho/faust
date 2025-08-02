use std::rc::Rc;

use implicit_clone::ImplicitClone;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::hooks::ModelHandle;

#[autoprops]
#[function_component(OptionGroup)]
pub fn option_group<T = AttrValue>(
    children: &ChildrenWithProps<OptionButton<T>>,
    #[prop_or_default] class: Classes,
    #[prop_or_default] title: Option<AttrValue>,
    name: AttrValue,
    value: ModelHandle<T>,
) -> Html
where
    T: PartialEq + Clone + ImplicitClone + 'static,
{
    let options = children.iter().map(|mut child| {
        let props = Rc::make_mut(&mut child.props);
        let checked = props.value == *value;
        props.name = name.clone();
        props.checked = checked;
        props.onchange = {
            let props = props.clone();
            let value = value.clone();
            Callback::from(move |_| value.set(props.value.clone()))
        };
        child
    });

    html! {
      <div {class} {title}>
        {for options}
      </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct OptionButtonProps<T>
where
    T: PartialEq,
{
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<()>,
    pub value: T,
}

#[function_component(OptionButton)]
pub fn option_button<T = AttrValue>(props: &OptionButtonProps<T>) -> Html
where
    T: PartialEq + Clone + ImplicitClone,
{
    let OptionButtonProps {
        children,
        class,
        title,
        name,
        checked,
        onchange,
        ..
    } = props.clone();
    let onchange = onchange.reform(|_| ());
    html! {
      <label {class} {title}>
        <input type="radio" {name} {checked} {onchange}/>
        {children}
      </label>
    }
}
