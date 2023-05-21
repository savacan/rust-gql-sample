use actix_service::{Service, Transform};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use anyhow::Result;
use futures_util::{
    future::{self, LocalBoxFuture},
    FutureExt,
};
use sample_sql::{MySqlPool, User};
use std::rc::Rc;

pub struct UserAuthentication {
    pool: MySqlPool,
}
impl UserAuthentication {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for UserAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = UserAuthMiddleware<S>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(UserAuthMiddleware {
            service: Rc::new(service),
            pool: self.pool.clone(),
        })
    }
}
pub struct UserAuthMiddleware<S> {
    service: Rc<S>,
    pool: MySqlPool,
}

impl<S, B> Service<ServiceRequest> for UserAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, std::result::Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let pool = self.pool.clone();
        async move {
            let user_id = req
                .headers()
                .get("user_id")
                .and_then(|k| k.to_str().ok())
                .and_then(|k| i64::from_str_radix(k, 10).ok());

            if let Some(user_id) = user_id {
                if let Ok(user) = User::find_by_id(&pool, user_id).await {
                    req.extensions_mut().insert(user);
                };
            }
            service.call(req).await.map(|res| res.map_into_left_body())
        }
        .boxed_local()
    }
}
