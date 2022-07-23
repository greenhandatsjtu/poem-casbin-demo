mod auth;

use poem::web::Data;
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};
use poem_casbin_auth::casbin::function_map::key_match2;
use poem_casbin_auth::casbin::{CoreApi, DefaultModel, FileAdapter};
use poem_casbin_auth::{CasbinService, CasbinVals};
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
    let m = DefaultModel::from_file("rbac_with_pattern_model.conf")
        .await
        .unwrap();
    let a = FileAdapter::new("rbac_with_pattern_policy.csv");

    let casbin_middleware = CasbinService::new(m, a).await.unwrap();

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    let app = Route::new()
        .at("/pen/1", get(pen1))
        .at("/pen/2", get(pen2))
        .at("/book/:id", get(book))
        .at("/user", get(user))
        .with(casbin_middleware)
        .with(auth::BasicAuth);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("poem-casbin-demo")
        .run(app)
        .await
}
