use introspect_types::{Attribute, ChildDefs, ColumnDef, PrimaryDef};

pub trait TableStructure {
    type Primary;
    type Record;
    fn attributes() -> Array<Attribute> {
        array![]
    }
    fn primary() -> PrimaryDef {
        introspect_types::PrimaryDef {
            name: "__id",
            type_def: introspect_types::PrimaryTypeDef::Felt252,
            attributes: [].span(),
        }
    }
    fn columns() -> Span<ColumnDef>;
    fn collect_child_defs(ref defs: ChildDefs) {}
}
