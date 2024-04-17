# Zero JSON (`zjson`)

**Parse JSON with zero allocation.**

## Status
This library is still in development. We still need:
- Tests
- Better documentation
- More examples
- Potentially a simplified API (it is not stable yet)

If you would like to contribute to this, please open an issue or [contact me directly](https://tomboddaert.com/contact).

## How to Use
Create a document from a JSON string.

Call `next` on any container (document, object, array) to get the next value.
The returned value must be fully parsed before continuing.

Match on the returned `Any` value to handle each type.

Call `get` on a single value (string, number, boolean, null) to get it.

Call `finish` on a value to skip it (so that the parent container can continue).

### Features
- `alloc` - adds features that require allocation (only allocating escaped strings, there is a no-alloc alternative)
- `std` (enables `alloc`, default) - adds features that require `std` (`Error` impls)

## Specification
The parser is (hopefully) [ECMA 404](https://ecma-international.org/publications-and-standards/standards/ecma-404/) complient, including support for Unicode surrogate pairs.
The [diagram on json.org](https://www.json.org/json-en.html) is a great representation of the standard.

## Alternatives
This library is designed to be fast and require no allocations.
This means that the API is more complex than something like [`serde`](https://crates.io/crates/serde) with [`serde_json`](https://crates.io/crates/serde_json).

## Examples
There are examples in the [examples](./examples) directory.

## License
Zero JSON is licensed under either the [MIT](./LICENSES/MIT) license or the [Apache License Version 2.0](./LICENSES/APACHE-2.0) at your option.
