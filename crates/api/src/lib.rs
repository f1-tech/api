#![allow(clippy::too_many_arguments)]

pub mod circuit;

type Backend = diesel::mysql::Mysql;

pub mod handlers {
    use crate::*;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        circuit::handlers().into_iter().collect()
    }
}
