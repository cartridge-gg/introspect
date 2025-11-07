use crate::{ColumnDef, TypeDef};

pub trait Schema<T> {
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}
