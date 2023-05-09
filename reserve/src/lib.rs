mod manager;

use abi::{Error, FilterPager, Reservation, ReservationFilter, ReservationID, ReservationQuery};
use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub struct ReserveManager {
    pool: PgPool,
}

#[async_trait]
pub trait Reserve {
    /// make an reservation
    async fn reserve(&self, rs: Reservation) -> Result<Reservation, Error>;
    /// delete target reservation
    async fn delete(&self, id: ReservationID) -> Result<Reservation, Error>;
    /// modify reservation
    async fn modify(&self, rs: Reservation) -> Result<Reservation, Error>;
    /// query reservation by id
    async fn get(&self, id: ReservationID) -> Result<Reservation, Error>;
    /// query some reservations by condition
    async fn gets(&self, query: ReservationQuery) -> Receiver<Result<Reservation, Error>>;
    /// change target reservation status like: pending -> confirmed
    async fn change_status(&self, id: ReservationID) -> Result<Reservation, Error>;
    /// page query reservation
    async fn page(
        &self,
        query: ReservationFilter,
    ) -> Result<(FilterPager, Vec<Reservation>), Error>;
}
