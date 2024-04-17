use core::{cell::RefCell, fmt};

use zjson::{any::Any, document::Document, literal, number};

fn main() {
    let json = r#"{
        "one": 1,
        "array": [true, false, null],
        "object": {"pi": 3.14, "exp": 1e5}
    }"#;

    let mut document = Document::new(json);
    let root = document
        .next()
        .expect("failed to parse document")
        .expect("failed to get root from document");

    let format = Format(RefCell::new(root));
    println!("{format:?}");

    document.finish().expect("failed to parse document");
}

struct Format<'json, 'p>(RefCell<Any<'json, 'p>>);

impl<'json, 'p> fmt::Debug for Format<'json, 'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &mut *self.0.borrow_mut() {
            Any::String(string) => {
                let parsed = string.get().expect("failed to parse string");
                let raw = parsed.unescaped();
                write!(f, "{raw:?}")
            }

            Any::Number(number) => {
                let parsed = number.get().expect("failed to parse number");
                <number::ParsedNumber as fmt::Display>::fmt(&parsed, f)
            }

            Any::Object(object) => {
                let mut map = f.debug_map();

                while let Some((key, value)) = object.next().expect("failed to parse an object") {
                    let raw_key = key.unescaped();
                    map.entry(&raw_key, &Format(RefCell::new(value)));
                }

                map.finish()
            }

            Any::Array(array) => {
                let mut list = f.debug_list();

                while let Some(value) = array.next().expect("failed to parse an array") {
                    list.entry(&Format(RefCell::new(value)));
                }

                list.finish()
            }

            Any::Literal(literal) => {
                let parsed = literal.get().expect("failed to parse literal");
                <literal::ParsedLiteral as fmt::Display>::fmt(&parsed, f)
            }
        }
    }
}

// Test the example
#[test]
fn test() {
    main();
}
