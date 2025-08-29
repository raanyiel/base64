use crate::Command;
use crate::USAGE;
use lib_base64::Base64;

pub fn ced<T: AsRef<str>>(cmd: &Command, msg: T) {
    if msg.as_ref().is_empty() {
        eprintln!("{}", USAGE);
        return;
    }

    let msg = msg
        .as_ref()
        .replace("encoded text: ", "")
        .replace("decoded text: ", "");

    match cmd {
        Command::Encode => match msg.encode() {
            Ok(x) => println!("encoded text: {}", x),
            Err(e) => {
                eprintln!("error: {}", e);
                return;
            }
        },
        Command::Decode => match msg.decode() {
            Ok(x) => println!("decoded text: {}", x),
            Err(e) => {
                eprintln!("error: {}", e);
                return;
            }
        },
    }
}
