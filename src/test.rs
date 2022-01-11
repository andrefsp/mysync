use tokio::net::TcpStream;

use super::httpd::HttpServer;
use super::persistence::{Repo, DB};
use super::service::Svc;

pub fn new_svc() -> Svc {
    let db = DB::new();

    // Create repo.
    let repo = Repo::new(db);

    Svc::new(repo)
}

pub struct HttpTestServer {
    addr: String,
}

impl HttpTestServer {
    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn pick_addr() -> String {
        "127.0.0.1:3000".to_string()
    }

    pub async fn new(
        svc: Svc,
    ) -> Result<(HttpTestServer, Box<dyn FnOnce() + Sync + Send>), std::io::Error> {
        let (server, stop) = HttpServer::new(svc);

        let addr = HttpTestServer::pick_addr();

        let t_server = HttpTestServer { addr: addr.clone() };
        tokio::spawn(async move { server.start(addr.as_str()).await });

        for _ in 1..10 {
            let stm = TcpStream::connect(t_server.addr.clone()).await;
            match stm {
                Ok(_) => return Ok((t_server, stop)),
                _ => continue,
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "an error has occured",
        ))
    }
}
