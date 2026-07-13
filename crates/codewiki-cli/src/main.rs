//! CodeWiki command-line entrypoint.

fn main() {
    let output = codewiki_core::run(std::env::args().skip(1));

    if !output.stdout.is_empty() {
        print!("{}", output.stdout);
    }

    if !output.stderr.is_empty() {
        eprint!("{}", output.stderr);
    }

    if output.exit_code != 0 {
        std::process::exit(output.exit_code);
    }
}
