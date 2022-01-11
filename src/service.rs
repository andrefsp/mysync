use std::str;

use hyper::body::Body;
use hyper::Request;
use hyper::Response;

use routerify::Router;

use super::models::Car;
use super::persistence::Repo;

// we are going to instantiate a Svc structure at all requests
// therefore this struct must be Clone
#[derive(Clone)]
pub struct Svc {
    pub repo: Repo,
}

impl Svc {
    pub fn router(&self) -> Router<Body, hyper::Error> {
        // XXX(andrefsp) :: Don't know a more elegant way of doing this.
        let get_car_hnd = self.clone();
        let put_car_hnd = self.clone();

        Router::builder()
            .get("/:id", move |req| get_car_hnd.clone().get_car(req))
            .post("/", move |req| put_car_hnd.clone().put_car(req))
            .build()
            .unwrap()
    }

    pub async fn get_car(mut self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let count = self.repo.count().await;
        let b = Body::from(format!("{}", count));

        Ok(Response::builder().status(200).body(b).unwrap())
    }

    pub async fn put_car(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let body = req.into_body();

        let bytes = hyper::body::to_bytes(body).await?;
        let payload = str::from_utf8(&bytes).unwrap().to_string();

        let car = Car::from_json(payload);
        self.repo.add(car).await;

        Ok(Response::builder()
            .status(200)
            .body("PUT CAR".into())
            .unwrap())
    }

    pub fn new(repo: Repo) -> Svc {
        Svc { repo }
    }
}
