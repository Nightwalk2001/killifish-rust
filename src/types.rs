use std::sync::Arc;

use axum::Extension;
use mongodb::Collection;

use crate::errors::AppError;
use crate::mongo::{Person, Recording, Todo};

pub type HandledResult<T, E = AppError> = Result<T, E>;
pub type ApiResult<T, E = AppError> = Result<T, E>;

pub type SharedCollection<T> = Extension<Arc<Collection<T>>>;

pub type SharedPersons = SharedCollection<Person>;
pub type SharedRecordings = SharedCollection<Recording>;
pub type SharedTodos = SharedCollection<Todo>;
