use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    message: String,
}

fn main() {
    let options = Options::from_args();
    println!("{}", options.message);

    // Print fox
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
