pub mod column;
mod derive;
pub mod error;
pub mod id;
pub mod manager;
pub mod primary;
pub mod structure;
pub mod table;
pub mod templates;
pub mod utils;
pub use column::Column;
pub use error::{TableError, TableResult};
pub use id::IdVariant;
pub use primary::PrimaryAttribute;
pub use table::TableInterface;

pub const I_TABLE_PATH: &str = "introspect_table";
pub use introspect_macros::{IntrospectError, IntrospectResult};
