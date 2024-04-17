use zjson::{any::Any, document::Document, literal};

fn main() {
    let json = r#"{
        "one": 1,
        "array": [true, false, null],
        "object": {"pi": 3.14, "exp": 1e5, "ignore": "this"}
    }"#;

    let mut document = Document::new(json);

    // root
    let element = document.next().expect("failed to parse document");
    let Some(Any::Object(mut root)) = element else {
        panic!("failed to get an object from the document");
    };

    // "one"
    let element = root.next().expect("failed to parse object");
    let Some((key, Any::Number(mut one))) = element else {
        panic!("failed to get a number from the object");
    };
    let one = one.get().expect("failed to parse number");

    assert_eq!(key, "one");
    assert_eq!(one.as_u8(), Some(1));

    // "array"
    let element = root.next().expect("failed to parse object");
    let Some((key, Any::Array(mut array))) = element else {
        panic!("failed to get an array from the object");
    };

    assert_eq!(key, "array");

    // "array" -> 0
    let array_element = array.next().expect("failed to parse array");
    let Some(Any::Literal(mut r#true)) = array_element else {
        panic!("failed to get a false value from the array");
    };
    println!("{true:#?}");
    let r#true = r#true.get().expect("failed to parse a false value");

    assert_eq!(r#true, literal::ParsedLiteral::True);

    // skip the rest of "array"
    array.finish().expect("failed to parse array");

    // skip the rest of root
    root.finish().expect("failed to parse object");

    // finish document
    document.finish().expect("failed to parse document");
}

// Test the example
#[test]
fn test() {
    main();
}
