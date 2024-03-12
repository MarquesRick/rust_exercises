use ferris_says::say;
use std::io::{stdout, BufWriter};

//by default, all functions are private
//needed to put the word 'pub' to make it public
pub fn greet() {
    let stdout = stdout();
    let message = String::from("Hello world!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    let _ = say(&message, width, &mut writer);
}
