pub fn exit(code: i32) {
    println!("Exiting with code {}", code);
    std::process::exit(code);
}