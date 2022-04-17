pub use crate::barge::barge_proto;
pub use crate::routes;

pub fn routes_from_proto(routes: Vec<barge_proto::Route>) -> Vec<routes::Route> {
    routes.into_iter().map(|br| routes::Route::from(br)).collect()
}
