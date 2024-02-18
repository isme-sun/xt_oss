use std::sync::Arc;

use axum::{routing::get, Router};
use pages::{buckets, describe_regions};
use xt_oss::{oss, utils};

mod pages {
    use super::AppState;
    use std::sync::Arc;

    use axum::{extract::State, Json};
    use xt_oss::oss::entities::{bucket::ListAllMyBucketsResult, region::RegionInfoList};

    pub(super) async fn describe_regions(State(state): State<Arc<AppState<'_>>>) -> Json<RegionInfoList> {
        let result = state.oss_client.DescribeRegions().execute().await.unwrap();
        if let Ok(data) = result {
            Json(data.content())
        } else {
            panic!("error");
        }
    }

    pub(super) async fn buckets(State(state): State<Arc<AppState<'_>>>) -> Json<ListAllMyBucketsResult> {
        let result = state.oss_client.ListBuckets().execute().await.unwrap();
        if let Ok(data) = result {
            Json(data.content())
        } else {
            panic!("error");
        }
    }
}

#[derive(Debug)]
struct AppState<'a> {
    oss_client: oss::Client<'a>,
}

impl<'a> AppState<'a> {
    fn new() -> Self {
        let options = utils::options_from_env();
        let client = oss::Client::new(options);
        Self { oss_client: client }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let shared_state = Arc::new(AppState::new());
    let app = Router::new()
        .route("/", get(describe_regions))
        .route("/buckets", get(buckets))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
