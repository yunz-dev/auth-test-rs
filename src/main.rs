// an attribute telling the rust compiler to ignore warning on unused code
#![allow(unused)] 

// imports the Html struct
use axum::response::Html;
// imports router struct, used to define and manage routes
use axum::Router;
// the HTTP get function
use axum::routing::get;
// imports socketaddr struct from std library 
// used to represent an IP address and port combination
// for binding to the server
use std::net::SocketAddr;

// attribute used to run the main fucntion
// as a tokio asyc runtime
#[tokio::main]
// async allows you to use await
async fn main() {
    // create a new instance of router 
    let routes_hello = Router::new().route(
        // route specifications
        "/hello",
        // takes in a closure or lambda function
        get(|| async {Html("Hello <strong>World!</strong>")}),
        );

    // -- start server
    // 127.0.0.1 is the same as local host
    // creates a scoketaddr instance representing the ip address and port
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr) // binds server to this address
        // makes the routes_hello into a usable service
        .serve(routes_hello.into_make_service())
        // asyncly waits for the server to start and run
        .await
        // if an error occurs causes program to panic, change for prod environment
        .unwrap();
}

