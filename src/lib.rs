pub mod handlers;
pub mod httpd;
pub mod models;
pub mod persistence;
pub mod service;

// tests
#[cfg(test)]
mod service_test;

#[cfg(test)]
mod test;
