use super::test::new_svc;
use super::test::HttpTestServer;
use tokio;

#[tokio::test]
async fn start_and_close() {
    let svc = new_svc();
    let connect = HttpTestServer::new(svc).await;

    assert!(connect.is_ok());

    let (server, stop) = connect.unwrap();

    let client = hyper::Client::new();

    let url = format!("{}/mercedes", server.url());
    let resp = client.get(url.parse().unwrap()).await;

    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);

    stop() // stop the server at the end of the test.
}
