include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("Hello, world! from vectors.rs");
    println!("{}", message());
}
