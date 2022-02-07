fn main() {
    let message = std::env::args()
        .nth(1)
        .expect("Fox does not know what to say. Usage: foxsay <message>");
    println!("{}", message);
    println!(" \\");
    println!("  \\\n");
    println!("   /\\   /\\   Todd Vargo");
    println!("  //\\\\_//\\\\     ____");
    println!("  \\_     _/    /   /");
    println!("   / * * \\    /^^^]");
    println!("   \\_\\O/_/    [   ]");
    println!("    /   \\_    [   /");
    println!("    \\     \\_  /  /");
    println!("     [ [ /  \\/ _/");
    println!("    _[ [ \\  /_/");
}
