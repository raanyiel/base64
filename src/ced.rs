use crate::Command;
use crate::USAGE;
use lib_base64::Base64;

pub fn ced(cmd: &Command, msg: impl AsRef<str>) {
    if msg.as_ref().is_empty() {
        eprintln!("{}", USAGE);
        return;
    }
    let msg = msg.as_ref();
    let msg = if let Some(x) = msg.strip_prefix("encoded text: ") {
        x
    } else if let Some(x) = msg.strip_prefix("decoded text: ") {
        x
    } else {
        msg
    }
    .to_string();

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
