macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// cargo run --example c01-simple
// cargo watch -q -c -x "run -q --example c01-simple"

fn main() {
    say_hello!();
}