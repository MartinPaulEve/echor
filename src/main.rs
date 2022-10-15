//! A simple echo crate
use clap::{App, Arg};

/// Parses commandline arguments and echoes response.
///
/// This short method contains the whole program, which echoes back the
/// commandline arguments. It will also parse a "-n" option to omit a newline
/// character at the end of the output.
/// # Examples
///
/// Basic usage:
///
/// ```
/// echor "hello world"
/// ```
fn main() {
    // parse the command line
    let matches = App::new("echor")
        .version("0.0.1")
        .author("Martin Paul Eve <martin@eve.gd>")
        .about("An echo tool in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not output newlines")
                .takes_value(false)
        )
        .get_matches();

    // extract the arguments
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    let ending = if omit_newline {""} else {"\n"};

    // write to the console
    print!("{}{}", text.join(" "), ending);
}
