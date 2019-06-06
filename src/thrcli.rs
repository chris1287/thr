extern crate getopts;

mod thr;

use thr::parser::parse;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    parse(args[1..].to_vec());
}
