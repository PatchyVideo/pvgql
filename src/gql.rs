use juniper::FieldResult;
use juniper::RootNode;

use chrono::{DateTime, Utc};
use crate::models::Error;
use juniper::graphql_value;
#[derive(Clone)]
pub struct Context {
}

impl juniper::Context for Context {}

// use crate::common::PostResult;

// #[path="submit_handler/mod.rs"]
// mod submit_handler;
// use submit_handler::{NewCharacterSubmit, NewMusicSubmit, NewWorkSubmit, NewCPSubmit, NewPaperSubmit};

// #[path="result_query/mod.rs"]
// mod result_query;
// use result_query::{CharacterRankResult, Reasons, FilterConditions, SingleCharacterResult};

// #[path="user_manager/mod.rs"]
// mod user_manager;
// use user_manager::{SendVoteTokenInputs, LoginInputs, LoginResults};

use crate::services::listVideo;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
	// ------------------------------------------------
	//     listVideo
	// ------------------------------------------------
    pub fn listVideo(para: listVideo::ListVideoParameters) -> FieldResult<String> {
		Ok("aaa".into())

		//juniper::FieldError::new(e)
    }
}


pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
	Schema::new(QueryRoot {}, MutationRoot {})
}
