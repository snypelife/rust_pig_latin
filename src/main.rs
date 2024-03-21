use std::env;
use pig_latin::Config;
use pig_latin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sentence = Config::parse(args).sentence;
    let new_sentence: String = pig_latin::run(&sentence);
    println!("Input: {:?}", sentence);
    println!("Output: {:?}", new_sentence);
}
