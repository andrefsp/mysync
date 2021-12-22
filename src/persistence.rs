use std::collections::HashMap;
use async_trait::async_trait;
use super::models::Car;


// In order to support concurrency the trait must be Sync and Send
#[async_trait]
pub trait Persistence: Sync + Send {
    async fn get(&mut self, brand: String) -> Option<&Car>;
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

#[async_trait]
impl Persistence for DB {
    fn put(&mut self, car: Car) {
        self.items.insert(car.get_brand(), car);
    }

    async fn get(&mut self, brand: String) -> Option<&Car> {
        self.items.get(&brand)
    }

    fn len(&self) -> usize {
        self.items.len() 
    }
}


pub struct Repo {
    pe: Box<dyn Persistence>
}


impl Repo {
    pub fn new(pe: Box<dyn Persistence>) -> Repo {
        Repo{
            pe
        }
    }

    pub fn add(&mut self, car: Car) {
        self.pe.put(car);
    }

    pub async fn get(&mut self, brand: String) -> Option<&Car> {
        self.pe.get(brand).await
    }

    pub fn count(&mut self) -> usize {
        self.pe.len()
    }
}


