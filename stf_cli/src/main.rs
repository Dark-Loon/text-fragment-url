use clap::Parser;

fn main() {
    println!("Hello, world!");
}

/// Produce a URL that links directly to specific text in a web page.
#[derive(Parser, Debug)]
#[command(name = "stf")]
#[command(version = "1.0")]
#[command(about = "Produce a URL that links directly to specific text in a web page.")]
struct Cli {
    base: Option<String>,
    text: Option<String>,
    #[arg(short, long)]
    prefix: Option<String>,
    #[arg(short, long)]
    suffix: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Interactive,
    FromStdin { base: String, text: String },
    Direct { base: String, text: String },
}

#[derive(Debug, PartialEq)]
enum ModeError {
    MissingText,
    MissingBase,
}

#[cfg(test)]
mod tests {
    // use clap::CommandFactory;

    use super::*;

    // #[test]
    // fn verify_cli() {
    //     Cli::command().debug_assert();
    // }

    // #[test]
    // fn verify_less_than_three_arguments() {}
}
