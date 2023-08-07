#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

#[napi]
pub struct Foo {}

#[napi]
impl Foo {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        println!("new");
        Ok(Self {})
    }
}

fn main() {
    println!("Hello, world!")
}
