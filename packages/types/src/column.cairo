use crate::{ChildDefs, TypeDef};

pub trait ColumnDef<const META_SIZE: u32> {
    const SELECTOR: felt252;
    const META_DATA: [felt252; META_SIZE];
    type Type;
    impl TD: TypeDef<Self::Type>;
    const fn SIZE<impl TD: TypeDef<Self::Type>>() -> u32 {
        1 + META_SIZE + TD::SIZE
    }
    fn serialize(
        ref output: Array<felt252>,
    ) {
        output.append(Self::SELECTOR);
        output.append_span(Self::META_DATA.span());
        Self::TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        Self::TD::collect_children(ref children);
    }
    fn serialize_with_children(
        ref type_def: Array<felt252>, ref child_defs: ChildDefs,
    ) {
        type_def.append(Self::SELECTOR);
        type_def.append_span(Self::META_DATA.span());
        Self::TD::serialize_with_children(ref type_def, ref child_defs);
    }
}

