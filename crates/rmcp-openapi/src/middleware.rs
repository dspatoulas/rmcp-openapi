use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use reqwest_middleware::{Middleware, Next, reqwest::{Request, Response}};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone, Debug, Default)]
pub struct XrayInjectMiddleware;

#[async_trait::async_trait]
impl Middleware for XrayInjectMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response, reqwest_middleware::Error> {
        global::get_text_map_propagator(|p| {
            let cx = Span::current().context();
            p.inject_context(&cx, &mut HeaderInjector(req.headers_mut()));
        });
        next.run(req, extensions).await
    }
}
