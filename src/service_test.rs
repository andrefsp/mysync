use tokio;

use super::persistence::{DB, Repo};
use super::service::Svc;

use super::test::HttpTestServer;

async fn new_svc() -> Svc {
    let db = DB::new();

    // Create repo.
    let repo = Repo::new(db);

    Svc::new(repo)
}

#[tokio::test]
async fn start_and_close() { 
    let svc = new_svc().await;
    let connect = HttpTestServer::new(svc).await;
    
    assert!(connect.is_ok());

    let (server, stop) = connect.unwrap();

    let client = hyper::Client::new();

    let resp = client.get(server.url().parse().unwrap()).await;
  
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
    
    stop() // stop the server at the end of the test.
}
