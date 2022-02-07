use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    /// What does the fox say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the fox look dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Read the fox from a file
    foxfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let eye = if options.dead {
        "x".red().bold()
    } else {
        "*".bright_cyan()
    };

    println!("{}", options.message.bright_blue().underline());

    // Print fox
    match &options.foxfile {
        Some(path) => {
            let fox_template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {}", path.display()))?;
            println!("{}", fox_template.replace("{eye}", &eye));
        }
        None => {
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
    }

    Ok(())
}
