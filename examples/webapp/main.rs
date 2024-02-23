use std::{env, sync::Arc};

use axum::{routing::get, Router};
use tera::Tera;
use xt_oss::{oss, util};

mod pages {
    use super::AppState;
    use serde::Serialize;
    use std::sync::Arc;

    use axum::{extract::State, response::Html, Json};
    use tera::Context;
    use xt_oss::oss::entities::{bucket::ListAllMyBucketsResult, region::RegionInfoList};

    #[derive(Serialize)]
    struct User {
        name: String,
    }

    pub(crate) async fn index(State(state): State<Arc<AppState<'_>>>) -> Html<String> {
        let result = state
            .oss_client
            .ListBuckets()
            .execute()
            .await
            .unwrap()
            .unwrap();
        let buckets = result.content().buckets.bucket;
        let mut context = Context::new();
        context.insert("buckets", &buckets);
        Html(state.template.render("index.html", &context).unwrap())
    }

    pub(super) async fn describe_regions(
        State(state): State<Arc<AppState<'_>>>,
    ) -> Json<RegionInfoList> {
        let result = state.oss_client.DescribeRegions().execute().await.unwrap();
        if let Ok(data) = result {
            Json(data.content())
        } else {
            panic!("error");
        }
    }

    pub(super) async fn buckets(
        State(state): State<Arc<AppState<'_>>>,
    ) -> Json<ListAllMyBucketsResult> {
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
    template: Tera,
}

impl<'a> AppState<'a> {
    fn new() -> Self {
        let template_dir = {
            let mut root_dir = env::current_dir().unwrap();
            root_dir.push(env::var("WEBAPP_TEMPLATE_DIR").unwrap());
            root_dir.push("*.html");
            root_dir.display().to_string()
        };

        let options = util::options_from_env();
        let client = oss::Client::new(options);
        let tera = Tera::new(&template_dir).unwrap();
        Self {
            oss_client: client,
            template: tera,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let shared_state = Arc::new(AppState::new());
    let app = Router::new()
        .route("/", get(pages::index))
        .route("/describe", get(pages::describe_regions))
        .route("/buckets", get(pages::buckets))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
