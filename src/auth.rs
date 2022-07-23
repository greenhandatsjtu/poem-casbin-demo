use poem::{
    http::StatusCode,
    web::{
        headers,
        headers::{authorization::Basic, HeaderMapExt},
    },
    Endpoint, Error, Middleware, Request, Result,
};
use poem_casbin_auth::CasbinVals;

pub struct BasicAuth;

impl<E: Endpoint> Middleware<E> for BasicAuth {
    type Output = BasicAuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        BasicAuthEndpoint { ep }
    }
}

pub struct BasicAuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for BasicAuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        if let Some(auth) = req.headers().typed_get::<headers::Authorization<Basic>>() {
            let vals = CasbinVals {
                subject: String::from(auth.username()),
                domain: None,
            };
            req.extensions_mut().insert(vals);
            self.ep.call(req).await
        } else {
            Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}
