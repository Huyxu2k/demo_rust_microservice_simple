extern crate hyper;
extern crate futures;


#[macro_use]
extern crate log;
extern crate env_logger;

use tokio;
use std::convert::Infallible;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::pin::Pin;

use hyper::{Request,Response,StatusCode,Body,Error};
use hyper::server::Server;
use hyper::service::{Service, make_service_fn,service_fn};
use hyper::server::conn::AddrStream;



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


#[tokio::main]
async fn main(){
    //println!("Hello, world!");
    env_logger::init();
   
    let address=SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    info!("Running micreservice at {}",address);
    let make_svc = make_service_fn(|socket: &AddrStream| {
             let remote_addr = socket.remote_addr();
             async move {
                Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                     Ok::<_, Infallible>(
                         Response::new(Body::from(format!("Hello, {}!", remote_addr)))
                     )
                 }))
             }
         });
     let server=Server::bind(&address).serve(make_svc);  
    if let Err(e) = server.await {
                 eprintln!("server error: {}", e);
             }
}
