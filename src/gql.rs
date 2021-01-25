use juniper::FieldResult;
use juniper::RootNode;

use chrono::{DateTime, Utc};
use models::Rating;
use crate::{models};
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

use crate::services::{listVideo, getVideo, editTags, authorDB, playlist, users, rating};

pub struct Query;

#[juniper::graphql_object]
impl Query {
	fn apiVersion() -> &str {
		"1.0"
	}

	// ------------------------------------------------
	//     listVideo
	// ------------------------------------------------
	pub async fn listVideo(para: listVideo::ListVideoParameters) -> FieldResult<listVideo::ListVideoResult> {
		listVideo::listVideo_impl(para).await
	}
	// ------------------------------------------------
	//     getVideo
	// ------------------------------------------------
	pub async fn getVideo(para: getVideo::GetVideoParameters) -> FieldResult<models::Video> {
		getVideo::getVideo_impl(para).await
	}
	// ------------------------------------------------
	//     editTags
	// ------------------------------------------------
	pub async fn getTagObjects(para: editTags::GetTagObjectsBatchParameters) -> FieldResult<Vec<models::RegularTagObject>> {
		editTags::getTagObjectsBatch_impl(para).await
	}
	// ------------------------------------------------
	//     authorDB
	// ------------------------------------------------
	pub async fn getAuthor(para: authorDB::GetAuthorParameters) -> FieldResult<models::Author> {
		authorDB::getAuthor_impl(para).await
	}
	// ------------------------------------------------
	//     playlist
	// ------------------------------------------------
	pub async fn getPlaylist(para: playlist::GetPlaylistParameters) -> FieldResult<models::Playlist> {
		playlist::getPlaylist_impl(para).await
	}
	pub async fn listPlaylist(para: playlist::ListPlaylistParameters) -> FieldResult<playlist::ListPlaylistResult> {
		playlist::listPlatylist_impl(para).await
	}
	// ------------------------------------------------
	//     users
	// ------------------------------------------------
	pub async fn getUser(para: users::GetUserParameters) -> FieldResult<models::User> {
		users::getUser_impl(para).await
	}
	// ------------------------------------------------
	//     rating
	// ------------------------------------------------
	pub async fn getRating(para: rating::GetRatingParameters) -> FieldResult<Option<models::Rating>> {
		rating::getRating_impl(para).await
	}
}


pub struct Mutation;

#[juniper::graphql_object]
impl Mutation {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

pub struct Subscription;

#[juniper::graphql_object]
impl Subscription {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}


pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription>;

pub fn create_schema() -> Schema {
	Schema::new(Query {}, Mutation {}, juniper::EmptySubscription::new())
}
