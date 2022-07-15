use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use crate::types::HandledResult;

const MONGO_URI: &str = "mongodb://killifish:Wzw20010827@120.25.246.58:17017";

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub password: String,
    pub email: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Recording {
    tank: String,
    owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    genotype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sexual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,
    quantity: i8,
    trigger: String,
    time: String,
    succeed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub(crate) creator: String,
    pub(crate) content: String,
    pub(crate) create_at: DateTime<Utc>,
}

pub async fn setup() -> HandledResult<(
    Arc<Collection<Person>>,
    Arc<Collection<Recording>>,
    Arc<Collection<Todo>>
)> {
    let client = Client::with_uri_str(MONGO_URI).await?;

    let db = client.database("killifish");

    let persons = db.collection::<Person>("persons");
    let recordings = db.collection::<Recording>("recordings");
    let todos = db.collection::<Todo>("todos");

    Ok((
        Arc::new(persons),
        Arc::new(recordings),
        Arc::new(todos)
    ))
}
