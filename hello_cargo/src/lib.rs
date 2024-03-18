use ferris_says::say;
use std::io::{self, stdout, BufWriter};

//by default, all functions are private
//needed to put the word 'pub' to make it public
pub fn greet() {
    let stdout = stdout();
    let mut message_read = String::new();
    println!("{:-^40}", "Begin process 1 - Say Hello!");
    println!("Type a message and Ferris will repeat it: ");

    io::stdin()
        .read_line(&mut message_read)
        .expect("Error while reading..."); //if error, return expect message

    let width = message_read.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    let _ = say(&message_read, width, &mut writer);
}

pub fn calculate_numbers() {
    let mut numbers_typed = String::new();
    println!("{:-^40}", "Begin process 2 - Numbers!");
    let banner = "Type a sequence numbers\n\
                        separated by comma (,)\n\
                        e.g: 1, 2, 3";
    println!("{banner}");

    io::stdin()
        .read_line(&mut numbers_typed)
        .expect("Error reading console"); //if error, return expect message

    let nums: Vec<i32> = numbers_typed
        .split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();
    println!("Total is: {}", result);
}

pub fn get_process_selected() {
    let mut s = String::new();
    println!("{:-^40}", "Begin Process");
    println!("Type a option: (1) - Type numbers | (2) - Just say hello to Ferris");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    match s.trim().parse::<i32>().expect("Error at parsing to number") {
        1 => calculate_numbers(),
        2 => greet(),
        _ => greet(),
    }
    println!();
    println!("{}", "-".repeat(40));
}
