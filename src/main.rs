use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    /// What does the fox say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the fox look dead
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let eye = if options.dead { "x" } else { "*" };

    println!("{}", options.message);

    // Print fox
    println!(" \\");
    println!("  \\\n");
    println!("   /\\   /\\   Todd Vargo");
    println!("  //\\\\_//\\\\     ____");
    println!("  \\_     _/    /   /");
    println!("   / {eye} {eye} \\    /^^^]", eye = eye);
    println!("   \\_\\O/_/    [   ]");
    println!("    /   \\_    [   /");
    println!("    \\     \\_  /  /");
    println!("     [ [ /  \\/ _/");
    println!("    _[ [ \\  /_/");
}
