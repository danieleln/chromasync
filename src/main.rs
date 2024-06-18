mod cli;
mod config;

fn main() {
    let _matches = cli::build_parser().get_matches();
}
