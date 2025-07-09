use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use yew::{AttrValue, Properties};

#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Module {
    pub id: AttrValue,
    pub name: AttrValue,
    pub logo: AttrValue,
}
