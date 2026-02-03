use crate::{ChildDefs, TypeDef};

pub trait ColumnDef<const META_SIZE: u32> {
    const SELECTOR: felt252;
    const META_DATA: [felt252; META_SIZE];
    type Type;
    const fn SIZE<impl TD: TypeDef<Self::Type>>() -> u32 {
        1 + META_SIZE + TD::SIZE
    }
    fn serialize<impl TD: TypeDef<Self::Type>>(
        ref output: Array<felt252>,
    ) {
        output.append(Self::SELECTOR);
        output.append_span(Self::META_DATA.span());
        TD::serialize(ref output);
    }
    fn collect_children<impl TD: TypeDef<Self::Type>>(
        ref children: ChildDefs,
    ) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children<impl TD: TypeDef<Self::Type>>(
        ref output: Array<felt252>, ref children: ChildDefs,
    ) {
        output.append(Self::SELECTOR);
        output.append_span(Self::META_DATA.span());
        TD::serialize_with_children(ref output, ref children);
    }
}

