#![allow(non_snake_case)]

extern crate juniper;

use std::io;
use std::sync::Arc;
use std::env;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};

mod models;

#[macro_use]
mod common;
mod gql;
mod services;
//mod custom_scalar;

use std::os::raw::*;


use crate::gql::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
	let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}

async fn graphiql_handler() -> Result<HttpResponse, Error> {
    gqli_handler("/", None).await
}
async fn playground_handler() -> Result<HttpResponse, Error> {
    play_handler("/", None).await
}
async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &(), req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env::set_var("RUST_LOG", "info");
    //env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .data(create_schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(
                web::resource("/")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_handler)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}

// async fn graphql(
// 	st: web::Data<Arc<Schema>>,
// 	data: web::Json<GraphQLRequest>,
// ) -> Result<HttpResponse, Error> {
// 	let user = web::block(move || {
// 		let res = data.execute(&st, &());
// 		Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
// 	})
// 	.await?;
// 	Ok(HttpResponse::Ok()
// 		.content_type("application/json")
// 		.body(user))
// }

// #[actix_rt::main]
// async fn main() -> io::Result<()> {
// 	std::env::set_var("RUST_LOG", "actix_web=info");
// 	env_logger::init();

// 	// Create Juniper schema
// 	let schema = std::sync::Arc::new(create_schema());

// 	// Start http server
// 	HttpServer::new(move || {
// 		App::new()
// 			.data(schema.clone())
// 			.wrap(
// 				Cors::new() // <- Construct CORS middleware builder
// 					.allowed_origin("http://localhost:3000")
// 					.allowed_origin("http://127.0.0.1:3000")
// 					.allowed_origin("http://localhost:8080")
// 					.allowed_origin("http://127.0.0.1:8080")
// 					.max_age(3600)
// 					.finish()
// 			)
// 			.wrap(middleware::Logger::default())
// 			.service(web::resource("/graphql").route(web::post().to(graphql)))
// 			.service(web::resource("/graphiql").route(web::get().to(graphiql)))
// 	})
// 	.bind("0.0.0.0:8080")?
// 	.run()
// 	.await
// }

#[cfg(test)]
mod test {
	use std::convert::{TryFrom, TryInto};
	#[test]
	pub fn test_datetime() {
		// let val = serde_json::json!({
		// 	"created_at": {
		// 		"$date": { "$numberLong": 1602266939024i64 }
		// 	}
		// });
		// let obj2 = bson::Bson::try_from(val).unwrap();
		let json_date = serde_json::json!({ "$date": { "$numberLong": "1590972160292" } });
		let bson_date: bson::Bson = json_date.try_into().unwrap();
	}
}

