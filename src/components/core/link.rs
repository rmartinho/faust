use yew::prelude::*;
use yew_router::components::Link as RouterLink;

use crate::routes::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub classes: Classes,
    pub to: Route,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub anchor_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let LinkProps {
        classes,
        to,
        disabled,
        anchor_ref,
        children,
    } = props.clone();

    html! {
        <RouterLink<Route> {classes} role="button" {to} {disabled} anchor_ref={&anchor_ref} {children} />
    }
}
