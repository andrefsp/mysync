use std::sync::{Arc, Mutex};

use tokio;
use futures::future;
use hyper::Server;

use mysync::models::Car;
use mysync::service::MakeSvc;
use mysync::persistence::{DB, Repo};


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
        let svc = Arc::clone(&svc); // or let svc = svc.clone();
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
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    start_svc().await;
    Ok(())
}
