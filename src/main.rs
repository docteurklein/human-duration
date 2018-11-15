extern crate chrono;
extern crate chrono_humanize;

use std::env;
use chrono::Duration;
use chrono_humanize::{Accuracy, HumanTime, Tense};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    let duration: i64 = arg1.parse().unwrap();
    let dt = Duration::seconds(duration);
    let ht = HumanTime::from(dt);

    println!("{}", ht.to_text_en(Accuracy::Precise, Tense::Present));
}
