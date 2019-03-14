#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate inherit;
fn main() {}
fn a_func() {}
trait ATrait {
    fn do_something(&self) -> usize;
}
struct AnotherStruct {}
impl ATrait for AnotherStruct {
    fn do_something(&self) -> usize {
        0
    }
}