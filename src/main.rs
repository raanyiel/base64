#![feature(iterator_try_collect)]
use std::env;
use std::io::{self, IsTerminal};

mod ced;
use ced::ced;

enum Command {
    Encode,
    Decode,
}

const USAGE: &str = "usage: base64 {{encode|decode}} <string>";

fn main() {
    let args: Box<[String]> = env::args().collect::<Box<[String]>>();

    if args.len() <= 1 {
        eprintln!("{}", USAGE);
        return;
    }

    let cmd: Command = match &*args[1] {
        "encode" => Command::Encode,
        "decode" => Command::Decode,
        _ => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    if !io::stdin().is_terminal() {
        let msg = io::stdin()
            .lines()
            .try_collect::<Box<str>>()
            .expect("idk bruh");
        ced(&cmd, msg);
        return;
    }

    let (_, msg) = args.split_at(2);
    let msg = msg.join(" ");
    ced(&cmd, msg);
}
