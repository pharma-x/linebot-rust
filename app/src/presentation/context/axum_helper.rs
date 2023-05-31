use axum::{async_trait, extract::FromRequest, http::StatusCode, Json, RequestExt};
use hyper::Request;
use validator::Validate;

use crate::presentation::context::validate::ValidatedRequest;

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedRequest<J>
where
    B: Send + 'static,
    S: Send + Sync,
    J: Validate + 'static,
    Json<J>: FromRequest<(), B>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Ok(Json(data)) = req
            .extract::<Json<J>, _>()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"));
        data.validate()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"));
        Ok(Self(data))
    }
}
