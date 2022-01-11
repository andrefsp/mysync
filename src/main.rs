use tokio;

use futures::future;

use mysync::httpd::HttpServer;
use mysync::models::Car;
use mysync::persistence::{Repo, DB};
use mysync::service;

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

    // XXX(andrefsp):: Must keep a reference to `stop` otherwise will close
    // the server immediately after start.
    let (server, _stop) = HttpServer::new(svc);

    server.start("127.0.0.1:3000").await.ok();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_svc().await;
    Ok(())
}
