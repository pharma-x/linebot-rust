use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, Form, FromRequest},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};

use serde::de::DeserializeOwned;
use validator::Validate;

use crate::context::{errors::ServerError, validate::ValidatedRequest};

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
