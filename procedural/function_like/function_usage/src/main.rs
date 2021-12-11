#![allow(unused)]

extern crate function_macro;
use function_macro::make_answer;

struct A {
    a_attribute: i32
}
struct B {
    b_attribute: f32
}

make_answer!(A, B);

fn main() {
    //println!("{}", answer());
}
