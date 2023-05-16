use std::{ops::Deref, cmp::Ordering};

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UseVecStateHandle<T: Clone> {
    state: UseStateHandle<Vec<T>>
}

impl<T: Clone> Deref for UseVecStateHandle<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &*self.state
    }
}

impl<T: Clone> UseVecStateHandle<T> {
    pub fn set(&self, new_values: Vec<T>) {
        self.state.set(new_values)
    }

    pub fn sort_by<F: FnMut(&T, &T) -> Ordering>(&self, compare: F) {
        let mut new_vec = (*self.state).clone();
        new_vec.sort_by(compare);

        self.set(new_vec);
    }

    pub fn update<F: Fn(usize, &T) -> T>(&self, update: F) {
        let new_state = self.state.iter()
            .enumerate()
            .map(|(idx, item)| update(idx, item))
            .collect::<Vec<_>>();

        self.set(new_state);
    }

    pub fn update_single<F: Fn(&T) -> T>(&self, index: usize, update: F) {
        self.update(|idx, item| {
            if idx == index {
                update(item)
            } else {
                item.clone()
            }
        });
    }

    pub fn insert(&self, item: T) {
        let mut new_vec = (*self.state).clone();
        new_vec.push(item);

        self.set(new_vec);
    }

    pub fn insert_all(&self, items: Vec<T>) {
        let mut new_vec = (*self.state).clone();
        
        for item in items.into_iter() {
            new_vec.push(item);
        }

        self.set(new_vec);
    }

    pub fn remove(&self, index: usize) {
        let mut new_vec = (*self.state).clone();
        new_vec.remove(index);

        self.set(new_vec);
    }
}

#[hook]
pub fn use_vec_state<T: Clone + 'static, F: FnOnce() -> Vec<T>>(initializer: F) -> UseVecStateHandle<T> {
    let state = use_state(initializer);

    UseVecStateHandle { state }
}

#[hook]
pub fn use_vec_state_eq<T: Clone + Eq + 'static, F: FnOnce() -> Vec<T>>(initializer: F) -> UseVecStateHandle<T> {
    let state = use_state_eq(initializer);

    UseVecStateHandle { state }
}