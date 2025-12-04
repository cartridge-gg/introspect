use introspect_tests::{BytesIntoByteArray, ascii};


#[test]
fn print_ascii() {
    let value: ByteArray = ascii::CAIRO_UNDISPLAYABLE.span().into();
    println!("{:?}", value);
}
