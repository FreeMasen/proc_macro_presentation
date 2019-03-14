#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate inherit;
fn main() {}

trait Node {
    fn source_location(&self) -> (usize, usize);
}
struct Expr {
    source_location: (usize, usize),
}
impl Node for Expr {
    fn source_location(&self) -> (usize, usize) {
        self.source_location
    }
}
