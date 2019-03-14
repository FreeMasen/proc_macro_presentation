#[macro_use]
extern crate inherit;

#[derive(Nothing)]
struct AStruct {

}

#[nothing]
fn a_func() {

}

other_nothing! {

}

trait ATrait {
    fn do_something(&self) -> usize;
}

#[derive(ATrait)]
struct AnotherStruct {

}

trait Node {
    fn source_location(&self) -> (usize, usize);
}

#[inherit(Node)]
struct Expr {

}

fn first() {}
fn second() {}
fn main() {

}
bench!(first, second);