use ::minigrep::Config;
use std::env;
use std::process;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect(); // collect can return many types of collection so type annotation is required
                                                   // dbg!(args); //debug print whats in args

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    //Abstracted out
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read file");

    // println!("With text:\n{contents}");
}
