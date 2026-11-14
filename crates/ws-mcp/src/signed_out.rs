use async_trait::async_trait;
use ws_broker::{Error, Reader};
use ws_core::{Account, AccountId, Activity, IdempotencyKey, Placed, Position, Quote, Report, Security, SecurityId};

/// Answers every read with `Unauthorized`, for a server started before anyone signed in.
pub struct SignedOut;

#[async_trait]
impl Reader for SignedOut {
    async fn accounts(&self) -> Result<Vec<Account>, Error> {
        Err(Error::Unauthorized)
    }

    async fn positions(&self, _: &AccountId) -> Result<Vec<Position>, Error> {
        Err(Error::Unauthorized)
    }

    async fn activities(&self, _: &AccountId, _: usize) -> Result<Vec<Activity>, Error> {
        Err(Error::Unauthorized)
    }

    async fn search(&self, _: &str) -> Result<Vec<Security>, Error> {
        Err(Error::Unauthorized)
    }

    async fn quote(&self, _: &SecurityId) -> Result<Quote, Error> {
        Err(Error::Unauthorized)
    }

    async fn pending_orders(&self, _: &AccountId) -> Result<Vec<Placed>, Error> {
        Err(Error::Unauthorized)
    }

    async fn order(&self, _: &IdempotencyKey) -> Result<Report, Error> {
        Err(Error::Unauthorized)
    }
}
