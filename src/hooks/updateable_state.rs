use std::ops::Deref;

use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct UseUpdateableStateHandle<T: Clone> {
    state: UseStateHandle<T>
}

impl<T: Clone + PartialEq> PartialEq for UseUpdateableStateHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<T: Clone> Deref for UseUpdateableStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.state.deref()
    }
}

impl<T: Clone> UseUpdateableStateHandle<T> {
    pub fn set(&self, value: T) {
        self.state.set(value);
    }

    pub fn update<F: Fn(T) -> T>(&self, update: F) {
        let current_state = (*self.state).clone();
        let new = update(current_state);
        self.set(new);
    }
}

#[hook]
pub fn use_updateable_state<T: Clone + 'static, F: FnOnce() -> T>(init_fn: F) -> UseUpdateableStateHandle<T> {
    let state = use_state(init_fn);

    UseUpdateableStateHandle { state }
}

#[hook]
pub fn use_updateable_state_eq<T: Clone + Eq + 'static, F: FnOnce() -> T>(init_fn: F) -> UseUpdateableStateHandle<T> {
    let state = use_state_eq(init_fn);

    UseUpdateableStateHandle { state }
}