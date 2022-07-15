use axum::{Json, Router};
use axum::routing::{get, post};
use bson::doc;
use futures::StreamExt;
use mongodb::results::InsertOneResult;
use serde::{Serialize, Deserialize};
use crate::errors::AppError;
use crate::jwt::issue;
use crate::mongo::Person;
use crate::types::{ApiResult, SharedPersons};

pub fn router() -> Router {
    Router::new()
        .route("/api/persons", get(get_persons))
        .route("/api/signup", post(signup))
        .route("/api/signin", post(signin))
}

async fn signup(
    persons: SharedPersons,
    Json(person): Json<Person>,
) -> ApiResult<Json<InsertOneResult>> {
    let res = persons.insert_one(person, None).await?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct SigninReqBody {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct SigninRes {
    name: String,
    role: String,
    token: String,
}

async fn signin(
    persons: SharedPersons,
    Json(SigninReqBody { name, password }): Json<SigninReqBody>,
) -> ApiResult<Json<SigninRes>> {
    let filter = doc! {"name": &name};

    let mut person = persons
        .find_one(filter, None).await?
        .ok_or(AppError::Any)?;

    let token = issue(&person);

    Ok(Json(SigninRes {
        name,
        role: person.role,
        token,
    }))
}

async fn get_persons(
    persons: SharedPersons
) -> ApiResult<Json<Vec<Person>>> {
    println!("get persons");
    let mut cursors = persons.find(None, None).await?;

    let mut res: Vec<Person> = Vec::new();

    while let Some(person) = cursors.next().await {
        res.push(person?)
    }

    Ok(Json(res))
}
