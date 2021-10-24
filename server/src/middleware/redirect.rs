// Taken from https://github.com/MoonZoon/MoonZoon/blob/d46e47db01366be91890c3a50e0be35fa0072ad9/crates/moon/src/redirect.rs

use super::*;
use actix_web::dev::{forward_ready, ResponseBody, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::LOCATION;
use actix_web::http::uri::{Authority, InvalidUriParts, Scheme, Uri};
use actix_web::{Error, HttpResponse};
use bool_ext::BoolExt;
use futures::future::{ok, Either, FutureExt, LocalBoxFuture, Ready};
use std::convert::TryFrom;

/// Middleware for redirecting requests from HTTP to HTTPS.
///
/// It uses the `CONFIG.http_port` and `CONFIG.https_port`'s;
#[derive(Copy, Clone)]
pub struct ToHttps;

impl<S, B> Transform<S, ServiceRequest> for ToHttps
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<ResponseBody<B>>;
    type Error = S::Error;
    type Transform = ToHttpsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ToHttpsMiddleware { service })
    }
}

pub struct ToHttpsMiddleware<S> {
    service: S,
}

impl<S> ToHttpsMiddleware<S> {
    fn uri(req: &ServiceRequest) -> Result<Uri, InvalidUriParts> {
        let connection_info = req.connection_info();

        // Note: "http/1 does not send host in uri" (https://github.com/actix/actix-web/issues/1111)
        let mut uri_parts = req.uri().clone().into_parts();
        uri_parts.scheme = Scheme::try_from(connection_info.scheme()).ok();
        uri_parts.authority = Authority::try_from(connection_info.host()).ok();

        Uri::from_parts(uri_parts)
    }

    fn should_redirect(uri: &Uri) -> Option<()> {
        match (uri.scheme()?, uri.authority()?.port_u16()) {
            (_, Some(port)) => port == CONFIG.http_port,
            (scheme, None) => scheme == &Scheme::HTTP,
        }
        .to_option()
    }

    fn redirect_uri(uri: Uri) -> Option<Uri> {
        let mut uri_parts = uri.into_parts();

        if uri_parts.scheme.as_ref()? == &Scheme::HTTP {
            uri_parts.scheme = Some(Scheme::HTTPS);
        }
        uri_parts.authority = Authority::try_from(format!("{}:{}", uri_parts.authority?.host(), CONFIG.https_port).as_str()).ok();

        Uri::from_parts(uri_parts).ok()
    }

    fn redirect<B>(req: ServiceRequest, uri: &Uri) -> Ready<Result<ServiceResponse<ResponseBody<B>>, Error>> {
        let http_response = HttpResponse::MovedPermanently()
            .insert_header((LOCATION, uri.to_string()))
            .finish()
            .map_body(|_, body| ResponseBody::Other(body));

        ok(req.into_response(http_response))
    }
}

#[allow(clippy::type_complexity)]
impl<S, B> Service<ServiceRequest> for ToHttpsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<ResponseBody<B>>;
    type Error = S::Error;
    type Future = Either<LocalBoxFuture<'static, Result<Self::Response, Self::Error>>, Ready<Result<Self::Response, Self::Error>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Ok(uri) = Self::uri(&req) {
            if Self::should_redirect(&uri).is_some() {
                let redirect_uri = Self::redirect_uri(uri).unwrap();
                return Self::redirect(req, &redirect_uri).right_future();
            }
        }
        self.service
            .call(req)
            .map(|result| result.map(|response| response.map_body(|_, body| ResponseBody::Body(body))))
            .boxed_local()
            .left_future()
    }
}
