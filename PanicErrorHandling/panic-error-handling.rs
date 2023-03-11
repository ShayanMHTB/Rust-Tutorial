/**
 * Simple form of error handling in RUST
 *
 * @author S.M.
 */

fn drink(beverage: &str) {
  // You shouldn't drink too much sugary beverages.
  if beverage == "lemonade" {
    panic!("AAAaaa!!!");
  }

  println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
  drink("water");
  drink("lemonade");
}
