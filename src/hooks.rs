pub mod tables_hook;
pub mod vec_state;

pub mod prelude {
    pub use crate::hooks::tables_hook::*;
    pub use crate::hooks::vec_state::*;
}