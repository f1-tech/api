use std::convert::Infallible;
use std::marker::PhantomData;

use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::Router;
use http_body_util::BodyExt;
use tower::Service; // for `collect`

use crate::handlers;
use shared::prelude::{Pagination, Response, Series};

pub mod macros;
pub mod models;

pub struct Test<'a, T, U> {
    uri: &'a str,
    series: Series,
    pagination: Option<Pagination>,
    expected: T,
    target: PhantomData<U>,
}

impl<'a, T, U> Test<'a, T, U>
where
    U: serde::de::DeserializeOwned + std::fmt::Debug,
    T: PartialEq<U> + std::fmt::Debug,
{
    pub fn new(uri: &'a str, series: Series, expected: T) -> Self {
        Self {
            uri,
            series,
            pagination: None,
            expected,
            target: PhantomData,
        }
    }

    pub fn pagination(mut self, pagination: Option<Pagination>) -> Self {
        self.pagination = pagination;
        self
    }

    pub async fn test_ok(self) {
        let router = setup();

        let resp = get(router, self.uri).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let json: Response<U> = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.series, self.series);
        assert_eq!(json.pagination, self.pagination);
        assert_eq!(self.expected, json.data);
    }
}

pub fn setup() -> Router {
    use axum::routing::get;

    let pool = infrastructure::ConnectionPool::try_new().unwrap();

    let api_routes = Router::new()
        .route("/circuits", get(handlers::circuits::circuits))
        .route(
            "/constructors/standings",
            get(handlers::constructor_standings::constructor_standings),
        )
        .route("/constructors", get(handlers::constructors::constructors))
        .route(
            "/drivers/standings",
            get(handlers::driver_standings::driver_standings),
        )
        .route("/drivers", get(handlers::drivers::drivers))
        .route("/laps", get(handlers::laps::laps))
        .route("/races", get(handlers::races::races))
        .route("/pit-stops", get(handlers::pit_stops::pit_stops))
        .route("/seasons", get(handlers::seasons::seasons))
        .route("/status", get(handlers::status::status))
        .with_state(pool);

    Router::new().nest("/api/:series", api_routes)
}

pub async fn get(mut router: Router, uri: &str) -> Result<axum::http::Response<Body>, Infallible> {
    router
        .call(
            Request::builder()
                .uri(uri)
                .header("x-real-ip", "127.0.0.1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
}

pub fn parse_date(date: &str) -> chrono::NaiveDate {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

pub fn parse_time(time: &str) -> chrono::NaiveTime {
    chrono::NaiveTime::parse_from_str(time, "%H:%M:%S").unwrap()
}
