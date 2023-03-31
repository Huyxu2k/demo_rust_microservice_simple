extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

use log::{info};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{SocketAddr};
use anyhow;
use tokio;
use std::env;
use std::result::Result;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, StatusCode, Method, body};

use futures::future::Future;
use mysql_async::prelude::*;
use mysql_async::*;
use serde::{Deserialize,Serialize};

/**  Don't use
// struct Microservice;

// impl Service<Request<Vec<u8>>> for Microservice {
//     // type Request=Request;
//     type Response = Response<Vec<u8>>;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//      fn call(&mut self, request: Request<Vec<u8>>) -> Self::Future {
//         info!("Microservice received a request : {:?}", request);
//         //Box::new(futures::future::ok(Response::new("Hello")));
//         // create the body
//         let body: Vec<u8> = "hello, world!\n".as_bytes().to_owned();
//         // Create the HTTP response
//         let resp = Response::builder()
//             .status(StatusCode::OK)
//             .body(body)
//             .expect("Unable to create `http::Response`");

//         // create a response in a future.
//         let fut = async { Ok(resp) };

//         // Return the response as an immediate future
//         Box::pin(fut)
//     }

//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         todo!()
//     }
// }
*/

fn get_url()->String{
    if let Ok(url)=env::var("DATABASE_URL"){
       let otps=Opts::from_url(&url).expect("DATABASE_URL not found");
       if otps.db_name().expect("a database name is required").is_empty()
        {
            panic!("database name is empty");
        }
        url
    }
    else {
            "mysql://root:123456@127.0.0.1:3306/db_book".into()
    }
}
#[derive(Deserialize,Serialize,Debug)]
struct  Book {
    id: i32,
    book_name: String,
    amount :i32,
    author: String,
    note: String,
    category:String
}
// TODO: 31-03-2023
#[derive(Deserialize,Serialize,Debug)]
struct  BookDTO{
    book_name: String,
    amount :i32,
    author: String,
    note: String,
    category:String
}
impl  Book {
    fn new(id: i32,book_name:String,amount: i32,author:String,note:String,category:String,)->Self{
        Self {
            id,
            book_name,
            amount,
            author,
            note,
            category,
        }
    }
}
fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

async fn handle_request(req:Request<Body>,pool:Pool)->Result<Response<Body>,anyhow::Error>{
    match (req.method(),req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "The valid endpoints are /init /create_order /create_orders /update_order /orders /delete_order",
        ))),
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        (&Method::OPTIONS, "/init") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/create_book") => Ok(response_build(&String::from(""))),
        //(&Method::OPTIONS, "/create_books") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/update_book") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/delete_book") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/books") => Ok(response_build(&String::from(""))),

        (&Method::GET, "/init") => {
            let mut conn = pool.get_conn().await.unwrap();
            "DROP TABLE IF EXISTS book;".ignore(&mut conn).await?;
            "CREATE TABLE book (id INT AUTO_INCREMENT PRIMARY KEY, book_name VARCHAR(100),amount INT ,author VARCHAR(100),  note VARCHAR(500),category VARCHAR(100));".ignore(&mut conn).await?;
            drop(conn);
            Ok(response_build("{\"status\":true}"))
        }
        (&Method::POST, "/create_book") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream =hyper::body::to_bytes(req).await?;
            let book: BookDTO = serde_json::from_slice(&byte_stream).unwrap();

            "INSERT INTO book (book_name, amount,author, note, category) VALUES (:book_name, :amount, :author, :note,:category)"
                .with(params! {
                    "book_name" => book.book_name,
                    "amount" => book.amount,
                    "author" => book.author,
                    "note" => book.note,
                    "category" => book.category,
                })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_build("{\"status\":true}"))
        }
        // (&Method::POST, "/create_books") => {
        //     let mut conn = pool.get_conn().await.unwrap();

        //     let byte_stream = hyper::body::to_bytes(req).await?;
        //     let book: Vec<Book> = serde_json::from_slice(&byte_stream).unwrap();

        //     "INSERT INTO book (book_name, amount,author, describe, category) VALUES (:book_name, :amount, :author, :describe,:category)"
        //         .with(book.iter().map(|book| {
        //             params! {
        //             "book_name" => book.book_name,
        //             "amount" => book.amount,
        //             "author" => book.author,
        //             "describe" => book.describe,
        //             "category" => book.category,
        //             }
        //         }))
        //         .batch(&mut conn)
        //         .await?;

        //     drop(conn);
        //     Ok(response_build("{\"status\":true}"))
        // }

        (&Method::POST, "/update_book") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream =hyper::body::to_bytes(req).await?;
            let book: Book = serde_json::from_reader(&*byte_stream).unwrap(); //from_slice(&byte_stream).unwrap();

            "UPDATE book SET  book_name=:book_name, amount=:amount, author=:author, note=:note, category=:category WHERE id=:id"
                .with(params! {
                    "book_name" => book.book_name,
                    "amount" => book.amount,
                    "author" => book.author,
                    "note" => book.note,
                    "category" => book.category,
                })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_build("{\"status\":true}"))
        }

        (&Method::GET, "/books") => {
            let mut conn = pool.get_conn().await.unwrap();

            let book = "SELECT * FROM book"
                .with(())
                .map(&mut conn, |(id,book_name,amount,author,note,category,)| {
                    Book::new(
                        id,
                        book_name,
                        amount,
                        author,
                        note,
                        category,
                    )},
                ).await?;

            drop(conn);
            Ok(response_build(serde_json::to_string(&book)?.as_str()))
        }        
        
        (&Method::GET, "/delete_book") => {
            let mut conn = pool.get_conn().await.unwrap();

            let params: HashMap<String, String> = req.uri().query().map(|v| {
                url::form_urlencoded::parse(v.as_bytes()).into_owned().collect()
            }).unwrap_or_else(HashMap::new);
            let id = params.get("id");

            "DELETE FROM book WHERE id=:id"
                .with(params! { "id" => id, })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_build("{\"status\":true}"))
        }

        // Not Found
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
    }

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let url=get_url();
    let opts = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let constraints = PoolConstraints::new(5, 10).unwrap();//min 5, max 10 connection
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));


    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Running micreservice at {}", address);
    let make_svc = make_service_fn(|_| {
       let pool=pool.clone();
       async move{
        Ok::<_,Infallible>(service_fn(move |req| {
            let pool=pool.clone();
            handle_request(req, pool)
        }))
       }
    });
    let server = Server::bind(&address).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
