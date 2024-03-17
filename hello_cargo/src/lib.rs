use ferris_says::say;
use std::io::{self, stdout, BufWriter, Read};

//by default, all functions are private
//needed to put the word 'pub' to make it public
pub fn greet() {
    let stdout = stdout();
    let mut messageRead = String::new();
    let message = String::from("Hello world!");
    io::stdin()
        .read_line(&mut messageRead)
        .expect("Error while reading..."); //if error, return expect message

    let width = messageRead.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    let _ = say(&messageRead, width, &mut writer);
}
