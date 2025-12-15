fn main() {
    // get arguments at index 1 and 2, index 0 is the tool name cgrep
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("pattern: {:?}, path: {:?}", pattern, path)
}
