extern crate ws;

mod thr;

use thr::parser::parse;

fn handler(msg: &ws::Message) {
    match msg.as_text() {
        Ok(msg) => {
            let split = msg.split_whitespace();
            let mut vec: Vec<String> = Vec::new();
            for arg in split {
                vec.push(arg.to_string());
            }
            parse(vec);
        },
        Err(_) => {
            println!("invalid input");
        }
    };
}

fn main() {
    ws::listen("127.0.0.1:3012", |_| {
        move |msg| {
            handler(&msg);
            Ok(())
        }
    }).expect("cannot start server");
}
