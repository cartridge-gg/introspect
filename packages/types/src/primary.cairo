use crate::TypeDef;

pub trait PrimaryDef<const META_SIZE: u32> {
    const META_DATA: [felt252; META_SIZE];
    type Type;
    // const fn SIZE<impl TD: TypeDef<Self::Type, false>>() -> u32 {
    //     1 + META_SIZE + TD::SIZE
    // }
    fn serialize<impl TD: TypeDef<Self::Type>>(
        ref output: Array<felt252>,
    ) {
        output.append_span(Self::META_DATA.span());

        TD::serialize(ref output);
    }
}
