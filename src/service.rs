use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::str;

use futures::future;
use futures::executor::block_on;


use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use hyper::body::Body;
//use hyper::body::HttpBody;

use super::persistence::Repo;
use super::models::Car;


pub struct Svc {
    repo: Arc<Mutex<Repo>>,
}

impl Svc {

    pub async fn get_car(&self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let resp = Response::builder();
        let mut repo = self.repo.lock().unwrap();

        let count = repo.count();
        let b = Body::from(format!("{}", count));
        Ok(resp.status(200).body(b).unwrap())
    }

    pub async fn put_car(&self, req: Request<Body>) ->  Result<Response<Body>, hyper::Error> { 
        let body = req.into_body();

        let bytes = hyper::body::to_bytes(body).await?; 

        let payload = str::from_utf8(&bytes).unwrap().to_string();

        let car = Car::from_json(payload);
        
        let mut repo = self.repo.lock().unwrap();
        
        repo.add(car);

        let resp = Response::builder();
        Ok(resp.status(200).body("PUT CAR".into()).unwrap())
    }

    pub fn new(repo: Arc<Mutex<Repo>>) -> Svc {
        Svc{
            repo
        }
    }
}


impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future { 
        let resp = match (req.method(), req.uri().path()) {
            (&hyper::Method::GET, "/") => block_on(self.get_car(req)).unwrap(),
            (&hyper::Method::POST, "/") => block_on(self.put_car(req)).unwrap(),

            _ => Response::builder().status(200).body("".into()).unwrap(),
        };
        future::ok(resp)
    }
}



pub struct MakeSvc {
    repo: Arc<Mutex<Repo>>,
}


impl MakeSvc {
    pub fn new(repo: Arc<Mutex<Repo>>) -> MakeSvc {
        MakeSvc{
            repo
        }
    }
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let repo = self.repo.clone();
        Box::pin(async move {
            Ok(Svc::new( repo )) 
        })
    }
}
