use ferris_says::say;
use std::io::{self, stdout, BufWriter};

//by default, all functions are private
//needed to put the word 'pub' to make it public
pub fn greet() {
    let stdout = stdout();
    let mut message_read = String::new();

    println!("Type a message and Ferris will repeat it: ");

    io::stdin()
        .read_line(&mut message_read)
        .expect("Error while reading..."); //if error, return expect message

    let width = message_read.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    let _ = say(&message_read, width, &mut writer);
}
