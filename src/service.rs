use hyper::body::Body;

use routerify::Router;

use super::handlers::{GetCar, PutCar};
use super::persistence::Repo;

// In this implementation we are going to inject all the service
// properties into its handlers via clone.
pub struct Svc {
    repo: Repo,
}

impl Svc {
    pub fn router(&self) -> Router<Body, hyper::Error> {
        // Create the handlers here
        let get_car = GetCar::new(self.repo.clone());
        let put_car = PutCar::new(self.repo.clone());

        // hook handlers with appropriate URI
        Router::builder()
            .get("/:id", move |req| get_car.clone().handle(req))
            .post("/", move |req| put_car.clone().handle(req))
            .build()
            .unwrap()
    }

    pub fn new(repo: Repo) -> Svc {
        Svc { repo }
    }
}
