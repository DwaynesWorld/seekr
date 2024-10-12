use super::AppContext;
use axum::Router;

mod clusters;

pub(crate) fn router() -> Router<AppContext> {
    Router::new().merge(clusters::router())
}
