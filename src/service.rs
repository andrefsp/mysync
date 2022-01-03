use std::sync::{Arc, Mutex};
use std::str;
use std::net::SocketAddr;

use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::body::Body;

use hyper::service::{make_service_fn, service_fn};

use super::persistence::Repo;
use super::models::Car;


#[derive(Clone)]
pub struct Svc {
    repo: Arc<Mutex<Repo>>,
}


impl Svc {

    pub async fn get_car(&self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let mut repo = self.repo.lock().unwrap();

        let count = repo.count();
        let b = Body::from(format!("{}", count));

        let resp = Response::builder();
        Ok(resp.status(200).body(b).unwrap())
    }

    pub async fn put_car(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

        let body = req.into_body();

        let bytes = hyper::body::to_bytes(body).await?;
        let payload = str::from_utf8(&bytes).unwrap().to_string();

        let car = Car::from_json(payload);

        let mut repo = self.repo.lock().unwrap();

        repo.add(car);

        let resp = Response::builder();
        Ok(resp.status(200).body("PUT CAR".into()).unwrap())
    }

    // `handle` method takes ownership of the whole struct.
    pub async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::GET, "/") => self.get_car(req).await,
            (&hyper::Method::POST, "/") => self.put_car(req).await,

            _ => Ok(Response::new(Body::from("NOT FOUND")) ),
        }
    }

    pub fn new(repo: Arc<Mutex<Repo>>) -> Svc {
        Svc{
            repo,
        }
    }
}


pub async fn start(svc: Svc, addr: SocketAddr) -> Result<(), hyper::Error>{

    // A `MakeService` that produces a `Service` to handle each connection.
    let make_service = make_service_fn(move |_| {
        // We have to clone the context to share it with each invocation of
        // `make_service`. If your data doesn't implement `Clone` consider using
        // an `std::sync::Arc`.
        let svc = svc.clone();

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req| {
            svc.clone().handle(req)
        });

        // Return the service to hyper.
        async move { Ok::<_, hyper::Error>(service) }
    });

    let server = Server::bind(&addr).serve(make_service);

    match server.await {
        Err(err) => Err(hyper::Error::from(err)),
        _ => Ok(()),
    }
}
