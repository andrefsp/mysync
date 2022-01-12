use std::str;

use hyper::body::Body;
use hyper::Request;
use hyper::Response;

use routerify::ext::RequestExt;

use super::models::Car;
use super::persistence::Repo;

#[derive(Clone)]
pub struct GetCar {
    repo: Repo,
}

impl GetCar {
    pub fn new(repo: Repo) -> GetCar {
        GetCar { repo }
    }

    pub async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let id = req.param("id").unwrap();

        println!("requesting:: {}", id);
        let count = self.repo.count().await;
        let b = Body::from(format!("{}", count));

        Ok(Response::builder().status(200).body(b).unwrap())
    }
}

#[derive(Clone)]
pub struct PutCar {
    repo: Repo,
}

impl PutCar {
    pub fn new(repo: Repo) -> PutCar {
        PutCar { repo }
    }

    pub async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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
}
