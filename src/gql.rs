use editTags::{ListTagParameters, listTags_impl};
use juniper::{FieldResult, GraphQLSubscriptionValue};
use juniper::RootNode;

use chrono::{DateTime, Utc};
use notification::ListNotificationParameters;
use subscription::ListSubscriptionVideosParameters;
use crate::{models, services::{comment::{self, GetThreadParameters, Thread}, subscription}};
use crate::models::Error;
use juniper::graphql_value;

use crate::services::{authorDB, editTags, editVideo, getVideo, listVideo, notification, playlist, postvideo, rating, users};
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
	pub async fn getTagObjects(context: &Context, para: editTags::GetTagObjectsBatchParameters) -> FieldResult<Vec<models::TagObjectValue>> {
		editTags::getTagObjectsBatch_impl(context, para).await
	}
	pub async fn listTagObjects(context: &Context, para: editTags::ListTagParameters) -> FieldResult<editTags::ListTagsResult> {
		editTags::listTags_impl(context, para).await
	}
	// ------------------------------------------------
	//     authorDB
	// ------------------------------------------------
	pub async fn getAuthor(context: &Context, para: authorDB::GetAuthorParameters) -> FieldResult<authorDB::Author> {
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
	pub async fn getUser(context: &Context, para: users::GetUserParameters) -> FieldResult<users::User> {
		users::getUser_impl(context, para).await
	}
	pub async fn whoami(context: &Context) -> FieldResult<String> {
		users::whoami_impl(context).await
	}
	// ------------------------------------------------
	//     rating
	// ------------------------------------------------
	pub async fn getRating(context: &Context, para: rating::GetRatingParameters) -> FieldResult<Option<rating::Rating>> {
		rating::getRating_impl(context, para).await
	}
	// ------------------------------------------------
	//     subscriptions
	// ------------------------------------------------
	pub async fn listSubscriptions(context: &Context) -> FieldResult<Vec<subscription::Subscription>> {
		subscription::listSubscriptions_impl(context).await
	}
	pub async fn listSubscriptionVideos(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<subscription::ListSubscriptionVideosResult> {
		subscription::listSubscriptionVideos_impl(context, para).await
	}
	pub async fn listSubscriptionVideosRandomized(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<subscription::ListSubscriptionVideosResult> {
		subscription::listSubscriptionVideosRandomized_impl(context, para).await
	}
	// ------------------------------------------------
	//     notification
	// ------------------------------------------------
	pub async fn listNotifications(context: &Context, para: ListNotificationParameters) -> FieldResult<Vec<notification::NotificationObjectValue>> {
		notification::listNotification_impl(context, para).await
	}
	// ------------------------------------------------
	//     comment
	// ------------------------------------------------
	pub async fn getThread(context: &Context, para: GetThreadParameters) -> FieldResult<Thread> {
		comment::getThread_impl(context, para).await
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
	// ------------------------------------------------
	//     postvideo
	// ------------------------------------------------
	pub async fn postVideo(context: &Context, para: postvideo::PostVideoRequestData) -> FieldResult<postvideo::PostVideoResult> {
		postvideo::postVideo_impl(context, para).await
	}
	pub async fn batchPostVideo(context: &Context, para: postvideo::BatchPostVideoRequestData) -> FieldResult<postvideo::BatchPostVideoResult> {
		postvideo::batchPostVideo_impl(context, para).await
	}
	// ------------------------------------------------
	//     editVideo
	// ------------------------------------------------
	pub async fn editVideoTags(context: &Context, para: editVideo::EditVideoTagsParameters) -> FieldResult<Vec<models::TagObjectValue>> {
		editVideo::editVideoTags_impl(context, para).await
	}
	pub async fn setVideoClearence(context: &Context, para: editVideo::SetVideoClearenceParameters) -> FieldResult<i32> {
		editVideo::setVideoClearenceVideo_impl(context, para).await
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
