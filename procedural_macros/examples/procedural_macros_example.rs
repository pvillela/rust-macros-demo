#![allow(unused)]

use procedural_macros::print_and_replace;
use procedural_macros::trace;
use procedural_macros::MyMacro;

trait MyTrait {
    fn hello();
}

#[derive(MyMacro)]
struct MyStruct;

#[trace]
fn foo() {
    println!("`foo` executed");
}

#[trace(some_arg)]
fn bar() {}

#[trace(some_arg1, some_arg2)]
fn baz() {}

fn main() {
    MyStruct::hello();

    foo();

    // The print_and_replace macro generates the add function,
    // so the lines below  the next one won't compile without the macro execution.
    print_and_replace!(100);
    let x = add(1, 2);
    println!("{x}");
}
