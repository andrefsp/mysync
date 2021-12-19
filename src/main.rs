mod models;

use std::sync::{Arc, Mutex};

use tokio;
use futures::future;
use hyper::Server;

use models::Car;
use models::{DB, Repo, MakeSvc};


async fn start_svc() { 
    let addr = ([127, 0, 0, 1], 3000).into();

    let db = DB::new();
   
    // Repo must be moved into the Mutex::new()
    // Mutex is than moved into an Arc (Atomic Reference counter)
    let repo = Repo::new(Box::new(db));
    
    let svc = MakeSvc::new(Arc::new(Mutex::new(repo)));

    let server = Server::bind(&addr).serve(svc);
    
    server.await.unwrap();
}

async fn start(svc: &Arc<Mutex<Repo>>) {
    let mut spawns = Vec::new();
    let mut x = 1;
    while x <= 10 {
        let svc = Arc::clone(&svc);
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error +  Sync + Send>> {
    let db = DB::new();
   
    // Repo must be moved into the Mutex::new()
    // Mutex is than moved into an Arc (Atomic Reference counter)
    let service = Arc::new(Mutex::new(Repo::new(Box::new(db))));

    // Invoke start() with the service reference and await so that
    // all threads finish
    start(&service).await;

    // Check number of elements
    let svc = service.lock().unwrap();
    println!("Elements in the map: {}", svc.count());

    Ok(())
}
