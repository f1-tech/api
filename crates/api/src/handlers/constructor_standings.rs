use axum::extract::{Json, Path, Query, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn constructor_standings(
    pool: State<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetConstructorStandingsParameter>,
) -> Result<Json<Response<ConstructorStandingResponse>>> {
    let conn = &mut pool.from_series(series).get()?;

    let res = application::constructor_standings::ConstructorStandingsQueryBuilder::params(params)
        .query_and_count(conn)?;

    let response = Response {
        data: res.0.into(),
        pagination: Some(res.1),
        series,
    };

    Ok(Json(response))
}
