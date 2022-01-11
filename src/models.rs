use std::fmt::Display;
use std::fmt::Formatter;

use json;

#[derive(Clone)]
pub struct Car {
    brand: String,
}

impl Display for Car {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", &self.brand)
    }
}

impl Car {
    pub fn new(brand: String) -> Car {
        Car { brand }
    }

    pub fn get_brand(&self) -> String {
        self.brand.clone()
    }

    pub fn from_json(payload: String) -> Car {
        let obj = json::parse(payload.as_str()).unwrap();
        Car {
            brand: obj["brand"].to_string(),
        }
    }
}
