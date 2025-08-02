use std::ops::Deref;

use implicit_clone::ImplicitClone;
use yew::{html::IntoPropValue, prelude::*};

#[derive(PartialEq, Clone, ImplicitClone)]
pub struct ModelHandle<T> {
    value: T,
    setter: Callback<T>,
}

impl<T> ModelHandle<T> {
    pub fn new<Setter>(value: T, setter: Setter) -> Self
    where
        Setter: Fn(T) + 'static,
    {
        Self {
            value,
            setter: Callback::from(setter),
        }
    }

    pub fn set(&self, value: T) {
        self.setter.emit(value)
    }

    pub fn reduce<F>(&self, reduce_fn: F)
    where
        F: Fn(&T) -> T,
    {
        self.set(reduce_fn(self.deref()))
    }

    pub fn map<G, S, U>(&self, get_fn: G, set_fn: S) -> ModelHandle<U>
    where
        G: Fn(&T) -> U,
        S: Fn(U) -> T + 'static,
        T: Clone + 'static,
    {
        let model = self.clone();
        ModelHandle::new(get_fn(model.deref()), move |u| model.set(set_fn(u)))
    }
}

impl<T> ModelHandle<T>
where
    T: Clone + 'static,
{
    pub fn reduce_callback<A, F>(&self, reduce_fn: F) -> Callback<A>
    where
        F: Fn(&T) -> T + 'static,
    {
        let clone = self.clone();
        Callback::from(move |_| clone.reduce(&reduce_fn))
    }
}

impl<'a, T> From<&'a UseStateHandle<T>> for ModelHandle<T>
where
    T: Clone,
{
    fn from(state: &'a UseStateHandle<T>) -> Self {
        Self {
            value: (**state).clone(),
            setter: state.setter().to_callback(),
        }
    }
}

impl<T> From<UseStateHandle<T>> for ModelHandle<T>
where
    T: Clone,
{
    fn from(state: UseStateHandle<T>) -> Self {
        Self::from(&state)
    }
}

impl<T> IntoPropValue<ModelHandle<T>> for UseStateHandle<T>
where
    T: Clone,
{
    fn into_prop_value(self) -> ModelHandle<T> {
        ModelHandle::from(self)
    }
}

impl<T> IntoPropValue<ModelHandle<T>> for &'_ UseStateHandle<T>
where
    T: Clone,
{
    fn into_prop_value(self) -> ModelHandle<T> {
        ModelHandle::from(self)
    }
}

impl<T> Deref for ModelHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
