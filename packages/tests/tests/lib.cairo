use introspect_test_utils::{BytesIntoByteArray, ascii};


#[test]
fn print_ascii() {
    let value: ByteArray = ascii::CAIRO_UNDISPLAYABLE.span().into();
    println!("{:?}", value);
}
