use std::ops::Sub;

use axum::{extract::Query, Json, Router, routing::get};
use axum::extract::Path;
use bson::{doc, Document};
use chrono::{Duration, Utc};
use futures::StreamExt;
use mongodb::options::{CountOptions, FindOptions};
use serde::{Deserialize, Serialize};

use crate::mongo::Recording;
use crate::types::{ApiResult, SharedRecordings};

pub fn router() -> Router {
    Router::new()
        .route("/api/recordings/:id", get(get_recordings))
}

#[derive(Deserialize)]
struct Params {
    trigger: Option<String>,
    result: Option<String>,
    page: Option<i64>,
    pagesize: Option<i64>,
}

#[derive(Serialize)]
struct Response {
    recordings: Vec<Recording>,
    count: u64,
}

async fn get_recordings(
    recordings: SharedRecordings,
    Path(id): Path<String>,
    Query(Params { trigger, result, page, pagesize }): Query<Params>,
) -> ApiResult<Json<Response>> {
    let avi = Utc::now() - Duration::days(30);
    let mut filter = doc! {"tank": id, "time": {"$gte": avi.to_string()}};

    if let Some(t) = trigger {
        filter.insert("trigger", t.to_uppercase());
    }
    if let Some(r) = result {
        filter.insert("succeed", r.eq("success"));
    }

    let mut options = FindOptions::default();
    options.skip = Some(((page.unwrap_or(1) - 1) * pagesize.unwrap_or(10)) as u64);
    options.limit = Some(pagesize.unwrap_or(10));

    let count = recordings.count_documents(filter.clone(), None).await?;
    let mut cursors = recordings.find(filter, options).await?;

    let mut res: Vec<Recording> = Vec::new();

    while let Some(recording) = cursors.next().await {
        res.push(recording?)
    }

    Ok(Json(Response {
        recordings: res,
        count,
    }))
}
