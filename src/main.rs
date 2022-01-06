use std::net::SocketAddr;

use tokio;
use futures::future;

use mysync::models::Car;
use mysync::service;
use mysync::persistence::{DB, Repo};


async fn add_cars(repo: &Repo) {
    let mut spawns = Vec::new();
    let mut x = 1;

    while x <= 10 {
        let repo = repo.clone();
        let f = tokio::spawn(async move {

            let b = format!("brand-{}", x);
            let car = Car::new(String::from(b));

            repo.add(car).await;
        });
        spawns.push(f);
        x = x + 1;
    }

    future::join_all(spawns).await;
}


async fn start_svc() {

    let db = DB::new();

    // Create repo.
    let repo = Repo::new(db);

    add_cars(&repo).await;

    let svc = service::Svc::new(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    service::start(svc, addr).await.unwrap();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_svc().await;
    Ok(())
}
