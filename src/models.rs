use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Car{
    brand: String
}

impl Display for Car {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", &self.brand)
    }
}

impl Car {
    pub fn new(brand: String) -> Car {
        Car{
            brand
        }
    }
}

// In order to support concurrency the trait must be Sync and Send
pub trait Persistence: Sync + Send {
    fn get(&mut self, brand: String) -> Option<&Car>;
    fn put(&mut self, car: Car);
    fn len(&self) -> usize;
}

pub struct DB {
    items: HashMap<String, Car>
}

impl DB {
    pub fn new() -> DB {
        DB{
            items: HashMap::new(),
        }
    }
}

impl Persistence for DB {
    fn put(&mut self, car: Car) {
        self.items.insert(car.brand.clone(), car);
    }

    fn get(&mut self, brand: String) -> Option<&Car> {
        self.items.get(&brand)
    }

    fn len(&self) -> usize {
        self.items.len() 
    }
}


pub struct Service {
    pe: Box<dyn Persistence>
}


impl Service {
    pub fn new(pe: Box<dyn Persistence>) -> Service {
        Service{
            pe
        }
    }

    pub fn add(&mut self, car: Car) {
        self.pe.put(car);
    }

    pub fn get(&mut self, brand: String) -> Option<&Car> {
        self.pe.get(brand)
    }

    pub fn count(&self) -> usize {
        self.pe.len()
    }
}
