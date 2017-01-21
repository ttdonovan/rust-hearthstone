#[macro_use]
extern crate log;
extern crate env_logger;

extern crate hearthstone;

use hearthstone::cardxml;

fn main() {
    env_logger::init().unwrap();

    let cards = cardxml::load();

    println!("Cards: {:?}", cards.len());

    println!("First 10 Cards:");

    for card in cards.values().take(10) {
      println!("{:?}", card);
    }
}
