use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use validator::Validate;

use crate::validation::ValidRejection;

#[derive(Debug, Clone, Copy, Default)]
pub struct Valid<T>(pub T);

#[async_trait]
impl<State, Extractor> FromRequest<State> for Valid<Extractor>
where
    State: Send + Sync,
    Extractor: Validate + FromRequest<State>,
{
    type Rejection = ValidRejection<<Extractor as FromRequest<State>>::Rejection>;

    async fn from_request(req: Request, state: &State) -> Result<Self, Self::Rejection> {
        let inner = Extractor::from_request(req, state)
            .await
            .map_err(ValidRejection::Extractor)?;
        inner.validate()?;
        Ok(Valid(inner))
    }
}
