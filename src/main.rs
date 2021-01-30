#![allow(non_snake_case)]

extern crate juniper;

use std::io;
use std::sync::Arc;
use std::env;

use actix_cors::Cors;
use actix_web::{App, Error, HttpMessage, HttpResponse, HttpServer, client::ClientBuilder, cookie, middleware, web};
use context::Context;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper_actix::{
	graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};

mod context;
mod models;

#[macro_use]
mod common;
mod gql;
mod services;
//mod custom_scalar;

use std::os::raw::*;


use crate::gql::{create_schema, Schema};

async fn graphiql_handler() -> Result<HttpResponse, Error> {
	gqli_handler("/graphql", None).await
}
async fn playground_handler() -> Result<HttpResponse, Error> {
	play_handler("/graphql", None).await
}
async fn graphql(
	req: actix_web::HttpRequest,
	payload: actix_web::web::Payload,
	schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
	println!("handling gql request");
	let session = req.cookie("session").map(|f| f.value().to_string());
	let ctx = Context {
		session: session
	};
	graphql_handler(&schema, &ctx, req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	//env::set_var("RUST_LOG", "info");
	//env_logger::init();
	println!("Listening on 127.0.0.1:5008");

	let server = HttpServer::new(move || {
		App::new()
			.data(create_schema())
			.wrap(
				Cors::default()
					.allow_any_origin()
					.allow_any_header()
					.allow_any_method()
					.supports_credentials()
					.max_age(3600),
			)
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
			.service(
				web::resource("/graphql")
					.route(web::post().to(graphql))
					.route(web::get().to(graphql)),
			)
			.service(web::resource("/playground").route(web::get().to(playground_handler)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
	});
	server.bind("127.0.0.1:5008").unwrap().run().await
}
