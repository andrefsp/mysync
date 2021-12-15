mod models;

use std::sync::{Arc, Mutex};

use tokio;
use futures::future;

use models::Car;
use models::{DB, Service};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new();
    let service = Service::new(Box::new(db));

    let mut spawns = Vec::new();

    let svc_arc = Arc::new(Mutex::new(service));

    let mut x = 1;
    while x < 10 {
        let svc = Arc::clone(&svc_arc);
        let f = tokio::spawn(async move {
            let b = format!("brand-{}", x);

            let car = Car::new(String::from(b));
            
            let mut svc = svc.lock().unwrap();
            svc.add(car);
        });
        spawns.push(f);
        x = x + 1;
    }

    future::join_all(spawns).await;

    println!("Hello, world::: ");

    Ok(())
}
