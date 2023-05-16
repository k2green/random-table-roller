pub mod tables_hook;
pub mod vec_state;
pub mod currency_state;
pub mod updateable_state;

pub mod prelude {
    pub use crate::hooks::tables_hook::*;
    pub use crate::hooks::vec_state::*;
    pub use crate::hooks::currency_state::*;
    pub use crate::hooks::updateable_state::*;
}