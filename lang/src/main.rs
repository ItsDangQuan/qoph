include!(concat!(env!("OUT_DIR"), "/lexer.rs"));

fn main() {
    next_token();
    println!("Compilation pipeline initialized.");
}
