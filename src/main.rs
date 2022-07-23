mod auth;

use poem::web::Data;
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};
use poem_casbin_auth::CasbinVals;
use std::env;

#[handler]
fn pen1() -> String {
    String::from("I'm pen 1")
}

#[handler]
fn pen2() -> String {
    String::from("I'm pen 2")
}

#[handler]
fn book(Path(id): Path<String>) -> String {
    format!("I'm book {}", id)
}

#[handler]
fn user(data: Data<&CasbinVals>) -> String {
    format!("Hello, {}", &data.subject)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem=debug");
    }
    let app = Route::new()
        .at("/pen/1", get(pen1))
        .at("/pen/2", get(pen2))
        .at("/book/:id", get(book))
        .at("/user", get(user))
        .with(auth::BasicAuth);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("poem-casbin-demo")
        .run(app)
        .await
}
