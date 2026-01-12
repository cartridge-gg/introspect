mod derive;
pub mod error;
mod table;
mod templates;
pub mod utils;
pub use error::{Result, TableError};
pub use table::Table;

pub const I_TABLE_PATH: &str = "introspect_table";
