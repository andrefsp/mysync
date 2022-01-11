use tokio::sync::oneshot::{channel, Receiver};

use hyper::Server;
use routerify::RouterService;

use super::service::Svc;

pub struct HttpServer {
    svc: Svc,
    stop_rx: Receiver<()>,
}

impl HttpServer {
    pub async fn start(self, addr: &str) -> Result<(), hyper::Error> {
        // We use router from the service to build out HttpServer.
        let service = RouterService::new(self.svc.router()).unwrap();

        let server = Server::bind(&addr.parse().unwrap())
            .serve(service)
            .with_graceful_shutdown(async {
                if let Err(err) = self.stop_rx.await {
                    println!("error while receiving signal. '{}'", err);
                }
            });

        match server.await {
            Err(err) => {
                println!("err:: {:?}", err);
                Err(hyper::Error::from(err))
            }
            _ => Ok(()),
        }
    }

    pub fn new(svc: Svc) -> (HttpServer, Box<dyn FnOnce() + Sync + Send + 'static>) {
        let (stop_tx, stop_rx) = channel::<()>();
        let stop_fn = move || {
            stop_tx.send(()).ok();
        };
        let stop_fn = Box::new(stop_fn);

        (HttpServer { svc, stop_rx }, stop_fn)
    }
}
