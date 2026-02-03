use crate::type_def::TypeDefInline;

pub trait PrimaryDef<const META_SIZE: u32> {
    const META_DATA: [felt252; META_SIZE];
    type Type;
    impl TD: TypeDefInline<Self::Type>;
    fn serialize(
        ref output: Array<felt252>,
    ) {
        output.append_span(Self::META_DATA.span());
        Self::TD::serialize(ref output);
    }
}
