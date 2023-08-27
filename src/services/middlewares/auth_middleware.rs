use std::{
    pin::Pin,
    task::{Context, Poll},
};

use hyper::Body;
use tonic::body::BoxBody;
use tower::{Layer, Service};

use crate::{
    application::managers::authn::authn_manager::{
        AuthnManagerImpl, AuthnManagerTrait, VerifyJwtTokenParams,
    },
    services::{
        extensions::user_context_req::{UserContext, UserGrpcReqExt},
        middlewares::utils::MiddlewareUtil,
    },
};

#[derive(Debug, Clone)]
pub struct AuthMiddlewareLayer {
    auth_manager: AuthnManagerImpl,
}

impl AuthMiddlewareLayer {
    pub fn new(auth_manager: AuthnManagerImpl) -> Self {
        AuthMiddlewareLayer { auth_manager }
    }
}

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddleware {
            inner: service,
            auth_manager: self.auth_manager.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    auth_manager: AuthnManagerImpl,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S> Service<hyper::Request<Body>> for AuthMiddleware<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let auth_manager = self.auth_manager.clone();

        Box::pin(async move {
            let mut user_context: Option<UserContext> = None;

            if let Some(token) = MiddlewareUtil::extract_token(&req) {
                match auth_manager
                    .verify_jwt_token(VerifyJwtTokenParams { token })
                    .await
                {
                    Ok(auth) => user_context = Some(auth.into()),
                    Err(_) => {
                        todo!("Handle error here")
                    }
                }
            };

            let ext = UserGrpcReqExt {
                uri: req.uri().clone(),
                user_context,
            };

            req.extensions_mut().insert(ext);

            let response = inner.call(req).await?;

            Ok(response)
        })
    }
}
