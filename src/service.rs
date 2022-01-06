use std::str;
use std::net::SocketAddr;

use tower::make::Shared;
use hyper::service::service_fn;

use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::body::Body;

use super::persistence::Repo;
use super::models::Car;

// we are going to instantiate a Svc structure at all requests
// therefore this struct must be Clone
#[derive(Clone)]
pub struct Svc {
    repo: Repo,
}


impl Svc {

    pub async fn get_car(&mut self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

        let count = self.repo.count().await;
        let b = Body::from(format!("{}", count));

        Ok(
            Response::builder()
            .status(200)
            .body(b)
            .unwrap()
        )
    }

    pub async fn put_car(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

        let body = req.into_body();

        let bytes = hyper::body::to_bytes(body).await?;
        let payload = str::from_utf8(&bytes).unwrap().to_string();

        let car = Car::from_json(payload);
        self.repo.add(car).await;

        Ok(
            Response::builder()
            .status(200)
            .body("PUT CAR".into())
            .unwrap()
        )
    }

    // `handle` method takes ownership of the whole struct.
    // this method is called at every request.
    pub async fn handle(mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::GET, "/") => self.get_car(req).await,
            (&hyper::Method::POST, "/") => self.put_car(req).await,

            _ => Ok(Response::new(Body::from("NOT FOUND")) ),
        }
    }

    pub fn new(repo: Repo) -> Svc {
        Svc{
            repo,
        }
    }
}


pub async fn start(svc: Svc, addr: SocketAddr) -> Result<(), hyper::Error>{

    // Create a `Service` for responding to the request.
    let make_service = Shared::new(service_fn(move |req| {
        // clone the service in order to handle the request.
        //
        // At every request the service is cloned and the request
        // is handled exclusively on the handle method which
        // takes owership of the request but also the wholse
        // Svc struct.
        svc.clone().handle(req)
    }));


    let server = Server::bind(&addr).serve(make_service);

    match server.await {
        Err(err) => Err(hyper::Error::from(err)),
        _ => Ok(()),
    }
}
