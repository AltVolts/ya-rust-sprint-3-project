// use tower::{Layer, Service};
// use std::task::{Context, Poll};
// use std::future::Future;
// use std::pin::Pin;
// use http::{Request, Response};
// use tonic::body::BoxBody;
// use tracing::{info, error};
// use std::fmt;
//
// #[derive(Clone)]
// pub struct GrpcLogLayer;
//
// impl<S> Layer<S> for GrpcLogLayer {
//     type Service = GrpcLogService<S>;
//
//     fn layer(&self, inner: S) -> Self::Service {
//         GrpcLogService { inner }
//     }
// }
//
// #[derive(Clone)]
// pub struct GrpcLogService<S> {
//     inner: S,
// }
//
// impl<S, B> Service<Request<B>> for GrpcLogService<S>
// where
//     S: Service<Request<B>, Response = Response<BoxBody>>,
//     S::Error: fmt::Debug,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }
//
//     fn call(&mut self, req: Request<B>) -> Self::Future {
//         let method = req.uri().path().to_string();
//         info!("→ gRPC request: {}", method);
//         let fut = self.inner.call(req);
//         Box::pin(async move {
//             match fut.await {
//                 Ok(resp) => {
//                     info!("← gRPC response: {} status: {}", method, resp.status());
//                     Ok(resp)
//                 }
//                 Err(e) => {
//                     error!("✗ gRPC error: {} error: {:?}", method, e);
//                     Err(e)
//                 }
//             }
//         })
//     }
// }
//
// // Реализуем NamedService, делегируя внутреннему сервису
// impl<S> tonic::server::NamedService for GrpcLogService<S>
// where
//     S: tonic::server::NamedService,
// {
//     const NAME: &'static str = S::NAME;
// }
