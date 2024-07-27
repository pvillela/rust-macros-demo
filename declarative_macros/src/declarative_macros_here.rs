#[macro_export]
macro_rules! greetings {
    (struct $x: ident {}) => {
        println!("Hello from struct {}", stringify!($x));
    };
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

#[macro_export]
macro_rules! add {
    ($a:expr)=>{
        $a
    };
    ($a:expr,$b:expr)=>{
        $a+$b
    };
    ($a:expr,$($b:tt)*)=>{
        $a+add!($($b)*)
    }
}
