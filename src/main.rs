use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => repl(),
        1 => run(&args[0]),
        _ => {
            println!("usage: arc [script]");
            std::process::exit(64);
        }
    }
}

/// Initialize the read-eval-print-loop (REPL).
fn repl() {

}

/// Run an Arc script located at the provided path.
fn run(path_to_script: &String) {

}