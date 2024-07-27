use declarative_macros::{add, greetings};

fn main() {
    greetings!("Earthly"); // Prints "Hello, Earthly"
    greetings! {
        struct G {} // Prints "Hello from struct G"
    };
    println!("{}", add!(1));
    println!("{}", add!(1, 2));
    println!("{}", add!(1, 2, 3));

    let a = 42;

    macro_rules! foo {
        ($x: expr) => {
            a + $x
        };
    }

    println!("{}", foo!(5));
}
