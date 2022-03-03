use editTags::{ListTagParameters, listTags_impl};
use juniper::{FieldResult, GraphQLSubscriptionValue};
use juniper::RootNode;

use chrono::{DateTime, Utc};
use notification::ListNotificationParameters;
use pvsubscription::ListSubscriptionVideosParameters;
use crate::common::EmptyJSON;
use crate::services::comment::{PostCommentParameters, PostCommentResponse};
use crate::services::notification::{MarkNotificationsReadParameters, SendDmParameters};
use crate::services::tags::{self, GetPopularTagsParameters, GetPopularTagsResult};
use crate::{models, services::{comment::{self, GetThreadParameters, Thread}, pvsubscription}};
use crate::models::Error;
use juniper::graphql_value;

use crate::services::{authorDB, editTags, editVideo, getVideo, listVideo, notification, playlist, postvideo, rating, users, stats, leaderboard};
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
	pub async fn getRelatedVideo(context: &Context, para: getVideo::GetRelatedVideoParameters) -> FieldResult<Vec<models::Video>> {
		getVideo::getRelatedVideo_impl(context, para).await
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
	pub async fn listAdjacentVideos(context: &Context, para: playlist::ListAdjacentVideosParameters) -> FieldResult<Vec<models::VideoRank>> {
		playlist::listAdjacentVideos_impl(context, para).await
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
	pub async fn listSubscriptions(context: &Context) -> FieldResult<Vec<pvsubscription::PVSubscription>> {
		pvsubscription::listSubscriptions_impl(context).await
	}
	pub async fn listSubscriptionVideos(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<pvsubscription::ListSubscriptionVideosResult> {
		pvsubscription::listSubscriptionVideos_impl(context, para).await
	}
	pub async fn listSubscriptionVideosRandomized(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<pvsubscription::ListSubscriptionVideosResult> {
		pvsubscription::listSubscriptionVideosRandomized_impl(context, para).await
	}
	// ------------------------------------------------
	//     notification
	// ------------------------------------------------
	pub async fn listNotifications(context: &Context, para: ListNotificationParameters) -> FieldResult<notification::ListNotificationGQLResult> {
		notification::listNotification_impl(context, para).await
	}
	pub async fn listUnreadNotificationsCount(context: &Context) -> FieldResult<notification::ListUnreadNotificationCountGQLResult> {
		notification::listUnreadNotificationCount_impl(context).await
	}
	// ------------------------------------------------
	//     comment
	// ------------------------------------------------
	pub async fn getThread(context: &Context, para: GetThreadParameters) -> FieldResult<Thread> {
		comment::getThread_impl(context, para).await
	}
	// ------------------------------------------------
	//     tags
	// ------------------------------------------------
	pub async fn getPopularTags(context: &Context, para: GetPopularTagsParameters) -> FieldResult<GetPopularTagsResult> {
		tags::getPopularTags_impl(context, para).await
	}
	// ------------------------------------------------
	//     stats
	// ------------------------------------------------
	pub async fn getStats(context: &Context) -> FieldResult<stats::Stats> {
		stats::getStats_impl(context).await
	}
	// ------------------------------------------------
	//     leaderboard
	// ------------------------------------------------
	pub async fn getLeaderboard(context: &Context, hrs: i32, k: i32) -> FieldResult<leaderboard::LeaderboardResult> {
		leaderboard::getLeaderboard_impl(context, hrs, k).await
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
	pub async fn editVideoTagIds(context: &Context, para: editVideo::EditVideoTagIdsParameters) -> FieldResult<Vec<models::TagObjectValue>> {
		editVideo::editVideoTagIds_impl(context, para).await
	}
	pub async fn setVideoClearence(context: &Context, para: editVideo::SetVideoClearenceParameters) -> FieldResult<i32> {
		editVideo::setVideoClearenceVideo_impl(context, para).await
	}
	// ------------------------------------------------
	//     notification
	// ------------------------------------------------
	pub async fn markAsRead(context: &Context, para: MarkNotificationsReadParameters) -> FieldResult<EmptyJSON> {
		notification::markNotificationsRead_impl(context, para).await
	}
	pub async fn sendDM(context: &Context, para: SendDmParameters) -> FieldResult<EmptyJSON> {
		notification::sendDM_impl(context, para).await
	}
	// ------------------------------------------------
	//     comment
	// ------------------------------------------------
	pub async fn postComment(context: &Context, para: comment::PostCommentParameters) -> FieldResult<PostCommentResponse> {
		comment::postComment_impl(context, para).await
	}
	pub async fn postReply(context: &Context, para: comment::PostReplyParameters) -> FieldResult<bool> {
		comment::postReply_impl(context, para).await
	}
	pub async fn editComment(context: &Context, para: comment::EditCommentParameters) -> FieldResult<bool> {
		comment::editComment_impl(context, para).await
	}
	pub async fn hideComment(context: &Context, cid: String) -> FieldResult<bool> {
		comment::editCommentOp_impl(context, comment::EditCommentOp::Hide, cid).await
	}
	pub async fn delComment(context: &Context, cid: String) -> FieldResult<bool> {
		comment::editCommentOp_impl(context, comment::EditCommentOp::Del, cid).await
	}
	pub async fn pinComment(context: &Context, cid: String, pin: bool) -> FieldResult<bool> {
		comment::editCommentOp_impl(context, comment::EditCommentOp::Pin(pin), cid).await
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
