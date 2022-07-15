// #[async_trait]
// impl<B> FromRequest<B> for User
//     where
//         B: Send,
// {
//
//     async fn from_request(req: &mut RequestParts<B>) -> HandledResult<Self> {
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req)
//                 .await
//                 .map_err(|err| Error::from(err))?;
//         let Extension(pool) = Extension::<PgPool>::from_request(req)
//             .await
//             .map_err(|err| Error::from(err))?;
//         let claims = jwt::verify(bearer.token())?;
//
//         Ok(User::find_by_id(claims.sub, &pool).await?)
//     }
// }

use axum::{extract::{FromRequest, RequestParts}, async_trait};
use crate::errors::AppError;
use crate::mongo::Person;

#[async_trait]
impl<B> FromRequest<B> for Person where B: Send {
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        todo!()
    }
}