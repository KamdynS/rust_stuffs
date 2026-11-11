fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_n) => println!("{input}"),
        Err(error) => println!("error: {error}"),
    }
}
