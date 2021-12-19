use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::task::Poll;
use std::task::Context;
use std::pin::Pin;

use std::sync::{Arc, Mutex};

use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use hyper::body::Body;

use futures::future;

pub struct Car{
    brand: String
}

impl Display for Car {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", &self.brand)
    }
}

impl Car {
    pub fn new(brand: String) -> Car {
        Car{
            brand
        }
    }
}

// In order to support concurrency the trait must be Sync and Send
pub trait Persistence: Sync + Send {
    fn get(&mut self, brand: String) -> Option<&Car>;
    fn put(&mut self, car: Car);
    fn len(&self) -> usize;
}

pub struct DB {
    items: HashMap<String, Car>
}

impl DB {
    pub fn new() -> DB {
        DB{
            items: HashMap::new(),
        }
    }
}

impl Persistence for DB {
    fn put(&mut self, car: Car) {
        self.items.insert(car.brand.clone(), car);
    }

    fn get(&mut self, brand: String) -> Option<&Car> {
        self.items.get(&brand)
    }

    fn len(&self) -> usize {
        self.items.len() 
    }
}


pub struct Repo {
    pe: Box<dyn Persistence>
}


impl Repo {
    pub fn new(pe: Box<dyn Persistence>) -> Repo {
        Repo{
            pe
        }
    }

    pub fn add(&mut self, car: Car) {
        self.pe.put(car);
    }

    pub fn get(&mut self, brand: String) -> Option<&Car> {
        self.pe.get(brand)
    }

    pub fn count(&self) -> usize {
        self.pe.len()
    }
}

pub struct Svc {
    repo: Arc<Mutex<Repo>>,
}

impl Svc {
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
        let rsp = Response::builder();

        let uri = req.uri();
        if uri.path() != "/" {
            let body = Body::from(Vec::new());
            let rsp = rsp.status(404).body(body).unwrap();
            return future::ok(rsp);
        }

        let body = Body::from(Vec::from(&b"heyo!"[..]));
        let rsp = rsp.status(200).body(body).unwrap();
        future::ok(rsp)
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
        let fut = async move { Ok(Svc { repo }) };
        Box::pin(fut)
    }
}
