use serde::Deserialize;
use std::collections::HashMap;
use toml;
#[derive(Debug, Deserialize)]
struct Toys {
    balls: u8,
    bricks: u8,
    cars: HashMap<String, Car>,
    trucks: HashMap<String, Truck>,
}

#[derive(Debug, Deserialize)]
struct Car {
    color: String,
    speed: u8,
}

#[derive(Debug, Deserialize)]
struct Truck {
    load: u16,
    hp: u16,
}

const TEXT: &str = r#"
balls = 5
bricks = 250
[cars]
  [cars.car1]
  color = "green"
  speed = 3
  [cars.car2]
  color = "red"
  speed = 10
[trucks]
  [trucks.truck1]
  load = 15
  hp = 670
  [trucks.truck2]
  load = 25
  hp = 800
"#;

fn main() {
    let toys: Toys = toml::from_str(TEXT).unwrap();
    println!("{:#?}", toys);
}
