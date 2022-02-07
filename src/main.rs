use atty::Stream;
use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    /// What does the fox say?
    message: Option<String>,

    #[structopt(short = "d", long = "dead")]
    /// Make the fox look dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Read the fox from a file
    foxfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();
    if atty::isnt(Stream::Stdin) {
        io::stdin().read_to_string(&mut message)?;
    } else {
        if let Some(m) = options.message {
            message = m
        } else {
            return Err(ExitFailure::from(failure::err_msg(
                "Fox has nothing to say",
            )));
        };
    };
    let eye = if options.dead {
        "x".red().bold()
    } else {
        "*".bright_cyan()
    };

    println!("{}", message.bright_blue().underline());

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
