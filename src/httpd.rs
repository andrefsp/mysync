use tokio::sync::oneshot::{channel, Receiver};
use tower::make::Shared;

use hyper::service::service_fn;
use hyper::Server;

use super::service::Svc;


pub struct HttpServer {
    svc: Svc,
    stop_rx: Receiver<()>,
}

impl HttpServer {

    pub async fn start(self, addr: &str) -> Result<(), hyper::Error> {
        let svc = self.svc.clone();

        let make_service = Shared::new(service_fn(move |req| {
            // clone the service in order to handle the request.
            //
            // At every request the service is cloned and the request
            // is handled exclusively on the handle method which
            // takes owership of the request but also the wholse
            // Svc struct.
            svc.clone().handle(req)
        }));

        let server = Server::bind(&addr.parse().unwrap())
            .serve(make_service)
            .with_graceful_shutdown(async {
                self.stop_rx.await.ok();
            });

        match server.await {
            Err(err) => Err(hyper::Error::from(err)),
            _ => Ok(()),
        }
    }

    pub fn new(svc: Svc) -> (HttpServer, Box<dyn FnOnce() + Sync + Send>) {
        let (stop_tx, stop_rx) = channel::<()>();
        let stop_fn = move || {
            stop_tx.send(()).ok();
        };
        let stop_fn = Box::new(stop_fn);

        (HttpServer{
            svc,
            stop_rx,
        }, stop_fn)
    }
}
