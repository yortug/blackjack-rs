use std::fmt;

enum Suit {
  Hearts,
  Diamonds,
  Clubs,
  Spades,
}

enum Rank {
  Ace,
  Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
  Jack, Queen, King,
}

struct Card {
  Suit,
  Rank,
}

fn main() {
  println!("Hello, world!");
}
