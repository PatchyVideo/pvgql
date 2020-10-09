#![allow(non_snake_case)]

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

#[macro_use]
mod models;
mod common;
mod gql;
mod services;

use std::os::raw::*;


use crate::gql::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
	let html = graphiql_source("http://127.0.0.1:8080/graphql");
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}

async fn graphql(
	st: web::Data<Arc<Schema>>,
	data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
	let user = web::block(move || {
		let res = data.execute(&st, &());
		Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
	})
	.await?;
	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(user))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// Create Juniper schema
	let schema = std::sync::Arc::new(create_schema());

	// Start http server
	HttpServer::new(move || {
		App::new()
			.data(schema.clone())
			.wrap(
				Cors::new() // <- Construct CORS middleware builder
					.allowed_origin("http://localhost:3000")
					.allowed_origin("http://127.0.0.1:3000")
					.allowed_origin("http://localhost:8080")
					.allowed_origin("http://127.0.0.1:8080")
					.max_age(3600)
					.finish()
			)
			.wrap(middleware::Logger::default())
			.service(web::resource("/graphql").route(web::post().to(graphql)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql)))
	})
	.bind("0.0.0.0:8080")?
	.run()
	.await
}