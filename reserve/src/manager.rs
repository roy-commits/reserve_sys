use crate::{Reserve, ReserveManager};
use abi::{
    Error, FilterPager, Reservation, ReservationFilter, ReservationID, ReservationQuery, Status,
};
use async_trait::async_trait;
use sqlx::postgres::types::PgRange;
use sqlx::types::chrono::DateTime;
use sqlx::Row;
use tokio::sync::mpsc::Receiver;

#[async_trait]
impl Reserve for ReserveManager {
    async fn reserve(&self, mut rs: Reservation) -> Result<Reservation, Error> {
        // todo: validate rs is legal
        let status = Status::from_i32(rs.status).unwrap_or(Status::Pending);

        // todo generate timestamp
        // let timestamp = PgRange::from_ref(DateTime::from())
        let insert_sql =
            "insert into rsvp.reservations (id, user_id, status, resource_id, timespan, note) \
        values ($1, $2, $3, $4, $5::reservation_status, $6)";
        let id = sqlx::query(insert_sql)
            .bind(&rs.id)
            .bind(&rs.user_id)
            .bind(&status.to_string())
            .bind(&rs.resource_id)
            .bind(&rs.notes)
            .fetch_one(&self.pool)
            .await?
            .get(0);
        rs.id = id;

        Ok(rs)
    }

    async fn delete(&self, id: ReservationID) -> Result<Reservation, Error> {
        todo!()
    }

    async fn modify(&self, rsvp: Reservation) -> Result<Reservation, Error> {
        todo!()
    }

    async fn get(&self, id: ReservationID) -> Result<Reservation, Error> {
        todo!()
    }

    async fn gets(&self, query: ReservationQuery) -> Receiver<Result<Reservation, Error>> {
        todo!()
    }

    async fn change_status(&self, id: ReservationID) -> Result<Reservation, Error> {
        todo!()
    }

    async fn page(
        &self,
        query: ReservationFilter,
    ) -> Result<(FilterPager, Vec<Reservation>), Error> {
        todo!()
    }
}
