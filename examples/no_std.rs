#![allow(clippy::approx_constant)]

use zjson::{any::Any, document::Document};

fn main() {
    let json = r#"{
        "one": 1,
        "array": [true, false, null],
        "object": {"pi": 3.14, "exp": 1e5, "ignore": "this"}
    }"#;

    let mut document = Document::new(json);

    // root
    let mut root = document
        .next()
        .expect("failed to parse document")
        .and_then(Any::object)
        .expect("failed to get an object from the document");

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
    let r#true = array
        .next()
        .expect("failed to parse array")
        .and_then(Any::literal)
        .expect("failed to get a true value from the array")
        .get()
        .expect("failed to parse a true value");

    assert_eq!(r#true, true);

    // "array" -> 1
    let r#false = array
        .next()
        .expect("failed to parse array")
        .and_then(Any::literal)
        .expect("failed to get a false value from the array")
        .get()
        .expect("failed to parse a false value");

    assert_eq!(r#false, false);

    // "array" -> 2
    let null = array
        .next()
        .expect("failed to parse array")
        .and_then(Any::literal)
        .expect("failed to get a null value from the array")
        .get()
        .expect("failed to parse a null value");

    assert_eq!(null, None);

    // finish "array"
    let array_element = array.next().expect("failed to parse array");
    assert!(array_element.is_none());

    // "object"
    let element = root.next().expect("failed to parse object");
    let Some((key, Any::Object(mut object))) = element else {
        panic!("failed to get an object from the object");
    };

    assert_eq!(key, "object");

    // "object" -> "pi"
    let object_element = object.next().expect("failed to parse inner object");
    let Some((key, Any::Number(mut pi))) = object_element else {
        panic!("failed to get a number from the inner object");
    };
    let pi = pi.get().expect("failed to parse a number");

    assert_eq!(key, "pi");
    assert_eq!(pi, 3.14);

    // "object" -> "exp"
    let object_element = object.next().expect("failed to parse inner object");
    let Some((key, Any::Number(mut exp))) = object_element else {
        panic!("failed to get a number from the inner object");
    };
    let exp = exp.get().expect("failed to parse a number");

    assert_eq!(key, "exp");
    assert_eq!(exp.as_f32(), 1e5);

    // skip the rest of "object"
    object.finish().expect("failed to parse inner object");

    // finish root
    root.finish().expect("failed to parse object");

    // finish document
    document.finish().expect("failed to parse document");
}

// Test the example
#[test]
fn test() {
    main();
}
