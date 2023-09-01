use serde::Deserialize;
use std::collections::HashMap;
use toml;

// Create a new struct `Cars` to encapsulate the HashMap
#[derive(Debug)]
struct Cars {
    data: HashMap<String, Car>,
}

// Implement `Deserialize` for `Cars` to handle the deserialization logic
impl<'de> Deserialize<'de> for Cars {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut data = HashMap::<String, Car>::deserialize(deserializer)?;

        // Iterate over the HashMap and set the name for each car based on its key
        for (key, mut car) in data.iter_mut() {
            car.name = Some(key.clone());
        }

        Ok(Cars { data })
    }
}

#[derive(Debug, Deserialize)]
struct Toys {
    balls: u8,
    bricks: u8,
    cars: Cars,
    trucks: HashMap<String, Truck>,
}

#[derive(Debug, Deserialize)]
struct Car {
    name: Option<String>,  // Make the name field optional
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
