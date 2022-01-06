use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use async_trait::async_trait;
use super::models::Car;

// In order to support concurrency the trait must be Sync and Send
#[async_trait]
pub trait Persistence: Sync + Send {
    async fn get(&self, brand: String) -> Option<Car>;
    async fn put(&self, car: Car);
    async fn len(&self) -> usize;
}

pub struct DB {
    items: Arc<Mutex<HashMap<String, Car>>>
}

impl DB {

    // DB constructor must return the appropriate persistence
    // implementation type.
    //
    // As Persistence is used in concurrent environments it must be wrapper
    // into an Arc(Atomic reference counter)
    pub fn new() -> Arc<Box<dyn Persistence>> {
        Arc::new(
            Box::new(
                DB{
                    items: Arc::new(Mutex::new(HashMap::new())),
                }
            )
        )
    }
}

#[async_trait]
impl Persistence for DB {
    async fn put(&self, car: Car) {
        let items = self.items.lock();
        let mut items = items.unwrap();
        
        items.insert(car.get_brand(), car);
    }

    async fn get(&self, brand: String) -> Option<Car> {
        let items = self.items.lock();
        let items = items.unwrap();

        match items.get(&brand) {
            Some(car) => Some(car.clone()),
            _ => None,
        }
    }

    async fn len(&self) -> usize {
        let items = self.items.lock();
        let items = items.unwrap();

        items.len() 
    }
}


#[derive(Clone)]
pub struct Repo {
    pe: Arc<Box<dyn Persistence>>
}


impl Repo {
    pub fn new(pe: Arc<Box<dyn Persistence>>) -> Repo {
        Repo{
            pe
        }
    }

    pub async fn add(&self, car: Car) {
        self.pe.put(car).await;
    }

    pub async fn get(&mut self, brand: String) -> Option<Car> {
        self.pe.get(brand).await
    }
    pub async fn count(&mut self) -> usize {
        self.pe.len().await
    }
}
