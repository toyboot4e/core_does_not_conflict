use anchor::core;

struct HelloWorld;

impl ::core::fmt::Debug for HelloWorld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Hello, world!")
    }
}

fn main() {
    println!("{:?}", HelloWorld);
}
