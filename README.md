# JsonFlex [![Build Status](https://travis-ci.org/nacika-ins/json_flex.svg)](https://travis-ci.org/nacika-ins/json_flex)

Flexibly Parse a JSON string

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.json_flex]
git = "https://github.com/nacika-ins/json_flex.git"
```

and this to your crate root:

```rust
extern crate json_flex;
use json_flex::{JsonFlex, JFObject, Unwrap};
```

## Example

```rust
extern crate json_flex;
#[warn(unused_imports)]
use json_flex::{JsonFlex, JFObject, Unwrap};

fn main() {
    let array = JsonFlex::decode(r#"[1,2,3,4]"#.to_owned());
    println!("{:?}", array);


    let array = JsonFlex::decode(r#"["1","2","3","4"]"#.to_owned());
    println!("{:?}", array[0].into_string());
}
```

## License

JsonFlex is released under the [MIT License][license].

[license]: LICENSE