extern crate rustc_serialize;

mod json_flex;
pub use json_flex::JsonFlex;
pub use json_flex::Unwrap;
pub use json_flex::JFObject;

#[cfg(test)]
mod test;