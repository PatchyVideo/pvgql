use editTags::{ListTagParameters, listTags_impl};
use juniper::{FieldResult, GraphQLSubscriptionValue};
use juniper::RootNode;

use chrono::{DateTime, Utc};
use models::{Rating};
use subscription::ListSubscriptionVideosParameters;
use crate::{models, services::subscription};
use crate::models::Error;
use juniper::graphql_value;

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
use crate::context::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
	fn apiVersion() -> &str {
		"1.0"
	}

	// ------------------------------------------------
	//     listVideo
	// ------------------------------------------------
	pub async fn listVideo(context: &Context, para: listVideo::ListVideoParameters) -> FieldResult<listVideo::ListVideoResult> {
		listVideo::listVideo_impl(context, para).await
	}
	// ------------------------------------------------
	//     getVideo
	// ------------------------------------------------
	pub async fn getVideo(context: &Context, para: getVideo::GetVideoParameters) -> FieldResult<models::Video> {
		getVideo::getVideo_impl(context, para).await
	}
	// ------------------------------------------------
	//     editTags
	// ------------------------------------------------
	pub async fn getTagObjects(context: &Context, para: editTags::GetTagObjectsBatchParameters) -> FieldResult<Vec<models::RegularTagObject>> {
		editTags::getTagObjectsBatch_impl(context, para).await
	}
	pub async fn listTagObjects(context: &Context, para: editTags::ListTagParameters) -> FieldResult<editTags::ListTagsResult> {
		editTags::listTags_impl(context, para).await
	}
	// ------------------------------------------------
	//     authorDB
	// ------------------------------------------------
	pub async fn getAuthor(context: &Context, para: authorDB::GetAuthorParameters) -> FieldResult<models::Author> {
		authorDB::getAuthor_impl(context, para).await
	}
	// ------------------------------------------------
	//     playlist
	// ------------------------------------------------
	pub async fn getPlaylist(context: &Context, para: playlist::GetPlaylistParameters) -> FieldResult<models::Playlist> {
		playlist::getPlaylist_impl(context, para).await
	}
	pub async fn listPlaylist(context: &Context, para: playlist::ListPlaylistParameters) -> FieldResult<playlist::ListPlaylistResult> {
		playlist::listPlatylist_impl(context, para).await
	}
	// ------------------------------------------------
	//     users
	// ------------------------------------------------
	pub async fn getUser(context: &Context, para: users::GetUserParameters) -> FieldResult<models::User> {
		users::getUser_impl(context, para).await
	}
	pub async fn whoami(context: &Context) -> FieldResult<String> {
		users::whoami_impl(context).await
	}
	// ------------------------------------------------
	//     rating
	// ------------------------------------------------
	pub async fn getRating(context: &Context, para: rating::GetRatingParameters) -> FieldResult<Option<models::Rating>> {
		rating::getRating_impl(context, para).await
	}
	// ------------------------------------------------
	//     subscriptions
	// ------------------------------------------------
	pub async fn listSubscriptions(context: &Context) -> FieldResult<Vec<models::Subscription>> {
		subscription::listSubscriptions_impl(context).await
	}
	pub async fn listSubscriptionVideos(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<subscription::ListSubscriptionVideosResult> {
		subscription::listSubscriptionVideos_impl(context, para).await
	}
	pub async fn listSubscriptionVideosRandomized(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<subscription::ListSubscriptionVideosResult> {
		subscription::listSubscriptionVideosRandomized_impl(context, para).await
	}
}


pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

pub struct Subscription;

#[juniper::graphql_object(Context = Context)]
impl Subscription {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

impl GraphQLSubscriptionValue for Subscription {
	
}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create_schema() -> Schema {
	Schema::new(Query {}, Mutation {}, Subscription {})
}
