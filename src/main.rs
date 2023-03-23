extern crate hyper;
extern crate futures;


#[macro_use]
extern crate log;
extern crate env_logger;

use std::pin::Pin;

use hyper::{Request,Response,StatusCode,Body,Error};
use hyper::server::Server;
use hyper::service::{Service, make_service_fn,service_fn};

use futures::future::Future;

struct Microservice;

impl  Service<Request<Vec<u8>>> for Microservice {
    //type Request=Request;
    type Response=Response<Vec<u8>>;
    type Error=hyper::Error;
    type Future= Pin<Box<dyn Future<Output = Result<Self::Response,Self::Error> >>>;


    fn call(&mut self, request: Request<Vec<u8>>)->Self::Future{
        //info!("Microservice received a request : {:?}",request);
        //Box::new(futures::future::ok(Response::new("Hello")));
         // create the body
         let body: Vec<u8> = "hello, world!\n"
             .as_bytes()
             .to_owned();
         // Create the HTTP response
         let resp = Response::builder()
             .status(StatusCode::OK)
             .body(body)
             .expect("Unable to create `http::Response`");

         // create a response in a future.
         let fut = async {
             Ok(resp)
         };

         // Return the response as an immediate future
         Box::pin(fut)
    }

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    let address="127.0.0.1:8000".parse().unwrap_or_default();
    let server=Server::bind(address);
                info!("Running micreservice at {}",address);
        
}
