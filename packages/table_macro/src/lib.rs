pub mod column;
mod derive;
pub mod error;
pub mod id;
pub mod primary;
pub mod schema;
pub mod table;
pub mod templates;
pub mod utils;
pub use column::Column;
pub use error::{TableError, TableResult};
pub use id::IdVariant;
pub use primary::PrimaryAttribute;
pub use table::Table;

pub const I_TABLE_PATH: &str = "introspect_table";
pub use introspect_macros::{IntrospectError, IntrospectResult};
