use crate::ColumnDef;

trait Schema<T> {
    fn columns() -> Span<ColumnDef>;
}
