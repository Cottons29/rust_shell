pub fn exit(code: i32) {
    println!("\nExiting with code {}\n", code);
    std::process::exit(code);
}