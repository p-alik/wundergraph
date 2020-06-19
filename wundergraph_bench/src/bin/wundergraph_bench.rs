#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(
    clippy::print_stdout,
    clippy::wrong_pub_self_convention,
    clippy::mut_mut,
    clippy::non_ascii_literal,
    clippy::similar_names,
    clippy::unicode_not_nfc,
    clippy::enum_glob_use,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::used_underscore_binding,
    clippy::cargo_common_metadata,
    clippy::dbg_macro,
    clippy::doc_markdown,
    clippy::filter_map,
    clippy::map_flatten,
    clippy::match_same_arms,
    clippy::needless_borrow,
    clippy::option_map_unwrap_or,
    clippy::option_map_unwrap_or_else,
    clippy::redundant_clone,
    clippy::result_map_unwrap_or_else,
    clippy::unnecessary_unwrap,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_dependencies
)]

use actix_web::web::{Data, Json};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use failure::Error;
use juniper::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use structopt::StructOpt;
use wundergraph::scalar::WundergraphScalarValue;

#[derive(Debug, StructOpt)]
#[structopt(name = "wundergraph_bench")]
struct Opt {
    #[structopt(short = "u", long = "db-url")]
    database_url: String,
    #[structopt(short = "s", long = "socket", default_value = "127.0.0.1:8000")]
    socket: String,
}

// actix integration stuff
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLData(GraphQLRequest<WundergraphScalarValue>);

#[derive(Clone)]
struct AppState {
    schema: Arc<wundergraph_bench::Schema<DbConnection>>,
    pool: Arc<Pool<ConnectionManager<DbConnection>>>,
}

fn graphiql() -> Result<HttpResponse, Error> {
    let html = graphiql_source("/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql(
    Json(GraphQLData(data)): Json<GraphQLData>,
    st: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let ctx = st.get_ref().pool.get()?;
    let res = data.execute(&st.get_ref().schema, &ctx);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res)?))
}

#[cfg(feature = "postgres")]
type DbConnection = diesel::pg::PgConnection;

#[cfg(feature = "sqlite")]
type DbConnection = diesel::sqlite::SqliteConnection;

#[cfg(feature = "mysql")]
type DbConnection = diesel::mysql::MysqlConnection;

#[allow(clippy::print_stdout)]
fn main() {
    let opt = Opt::from_args();
    let manager = ConnectionManager::<DbConnection>::new(opt.database_url);
    let pool = Pool::builder()
        .max_size((num_cpus::get() * 2 * 4) as u32)
        .build(manager)
        .expect("Failed to init pool");

    let query = wundergraph_bench::api::Query::default();
    let mutation = wundergraph_bench::api::Mutation::default();
    let schema = wundergraph_bench::Schema::new(query, mutation);

    let schema = Arc::new(schema);
    let pool = Arc::new(pool);
    let data = AppState { schema, pool };
    let url = opt.socket;

    // Start http server
    println!("Started http server: http://{}", url);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Logger::default())
            .route("/graphql", web::get().to(graphql))
            .route("/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/graphiql")
                    .finish()
            }))
    })
    .bind(&url)
    .expect("Failed to start server")
    .run()
    .unwrap();
}
