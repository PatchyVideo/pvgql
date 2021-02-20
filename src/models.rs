
extern crate serde_json;
use md5::{Md5, Digest};

use juniper::{
	graphql_interface,
	GraphQLObject,
	execute,
	parser::{ParseError, ScalarToken, Spanning, Token},
	serde::de,
	EmptyMutation, FieldResult, InputValue, Object, ParseScalarResult, RootNode, ScalarValue,
	Value, Variables,
};
use std::{cell::RefMut, fmt};

use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use crate::{context::Context, services::{comment::{self, Thread}, rating::Rating}};

use crate::services::users::User;

#[path="./custom_scalar.rs"]
mod custom_scalar;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
	pub code: String,
	pub aux: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestResult<T> {
	pub status: String,
	pub data: Option<T>
}

use serde::de::IntoDeserializer;
use serde::de::Deserializer;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MyObjectId {
	Oid(serde_json::Map<String, serde_json::Value>),
	Str(String)
}

impl MyObjectId {
	pub fn to_oid(&self) -> Option<ObjectId> {
		match self {
		    MyObjectId::Oid(o) => {
				match o.get("$oid") {
					Some(value) => {
						match value.as_str() {
							Some(s) => {
								match ObjectId::with_string(s) {
									Ok(oid) => Some(oid),
									Err(_) => None
								}
							}
							None => None
						}
					},
					None => None
				}
			},
		    MyObjectId::Str(s) => {
				if s.len() > 0 {
					match ObjectId::with_string(s) {
						Ok(oid) => Some(oid),
						Err(_) => None
					}
				} else {
					None
				}
			}
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
	pub created_at: BsonDateTime,
	pub created_by: Option<MyObjectId>,
	pub modified_at: Option<BsonDateTime>,
	pub modified_by: Option<MyObjectId>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Meta")]
impl Meta {
	pub fn created_at(&self) -> DateTime<chrono::Utc> {
		self.created_at.to_UtcDateTime()
	}
	pub fn modified_at(&self) -> Option<DateTime<chrono::Utc>> {
		self.modified_at.as_ref().map(|a| a.to_UtcDateTime())
	}
	pub async fn created_by(&self, context: &Context) -> FieldResult<Option<User>> {
		match self.created_by.as_ref() {
			Some(u) => {
				let u = users::getUser_impl(context, users::GetUserParameters {
					uid: match u.to_oid() {
						Some(oid) => oid.to_string(),
						None => { return Ok(None) }
					}
				}).await?;
				Ok(Some(u))
			},
			None => Ok(None)
		}
	}
	pub async fn modified_by(&self, context: &Context) -> FieldResult<Option<User>> {
		match self.modified_by.as_ref() {
			Some(u) => {
				let u = users::getUser_impl(context, users::GetUserParameters {
					uid: match u.to_oid() {
						Some(oid) => oid.to_string(),
						None => { return Ok(None) }
					}
				}).await?;
				Ok(Some(u))
			},
			None => Ok(None)
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsonDateTime
{
	#[serde(rename = "$date")]
	pub ts: i64
}
use chrono::offset::TimeZone;
impl BsonDateTime {
	pub fn to_UtcDateTime(&self) -> DateTime<chrono::Utc> {
		let mut num_secs = self.ts / 1000;
		let mut num_millis = self.ts % 1000;

		// The chrono API only lets us create a DateTime with an i64 number of seconds
		// and a u32 number of nanoseconds. In the case of a negative timestamp, this
		// means that we need to turn the negative fractional part into a positive and
		// shift the number of seconds down. For example:
		//
		//     date       = -4300 ms
		//     num_secs   = date / 1000 = -4300 / 1000 = -4
		//     num_millis = date % 1000 = -4300 % 1000 = -300
		//
		// Since num_millis is less than 0:
		//     num_secs   = num_secs -1 = -4 - 1 = -5
		//     num_millis = num_nanos + 1000 = -300 + 1000 = 700
		//
		// Instead of -4 seconds and -300 milliseconds, we now have -5 seconds and +700
		// milliseconds, which expresses the same timestamp, but in a way we can create
		// a DateTime with.
		if num_millis < 0 {
			num_secs -= 1;
			num_millis += 1000;
		};

		chrono::Utc.timestamp(num_secs, num_millis as u32 * 1_000_000)
			.into()
	}
}


#[derive(Clone, Serialize, Deserialize)]
pub struct VideoItem {
	pub cover_image: String,
	pub title: String,
	pub desc: String,
	pub placeholder: Option<bool>,
	pub rating: f64,
	pub repost_type: String,
	pub copies: Vec<ObjectId>,
	pub series: Vec<ObjectId>,
	pub site: String,
	pub thumbnail_url: String,
	pub unique_id: String,
	pub upload_time: BsonDateTime,
	pub url: String,
	pub user_space_urls: Option<Vec<String>>,
	pub utags: Vec<String>,
	pub views: i32
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Video Item")]
impl VideoItem {
	pub fn cover_image(&self) -> &String {
		&self.cover_image
	}
	pub fn title(&self) -> &String {
		&self.title
	}
	pub fn desc(&self) -> &String {
		&self.desc
	}
	pub fn placeholder(&self) -> &Option<bool> {
		&self.placeholder
	}
	pub fn rating(&self) -> &f64 {
		&self.rating
	}
	pub fn repost_type(&self) -> &String {
		&self.repost_type
	}
	pub fn site(&self) -> &String {
		&self.site
	}
	pub fn thumbnail_url(&self) -> &String {
		&self.thumbnail_url
	}
	pub fn unique_id(&self) -> &String {
		&self.unique_id
	}
	pub fn upload_time(&self) -> DateTime<chrono::Utc> {
		self.upload_time.to_UtcDateTime()
	}
	pub fn url(&self) -> &String {
		&self.url
	}
	pub fn user_space_urls(&self) -> &Option<Vec<String>> {
		&self.user_space_urls
	}
	pub fn utags(&self) -> &Vec<String> {
		&self.utags
	}
	pub fn views(&self) -> &i32 {
		&self.views
	}
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="TagCategoryItem")]
pub struct TagCategoryItem {
	pub key: String,
	pub value: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlaylistMeta {
	pub cover: String,
	pub videos: i32,
	pub desc: String,
	pub private: bool,
	pub privateEdit: bool,
	pub title: String,
	pub views: i32
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="PlaylistMeta")]
impl PlaylistMeta {
	pub fn cover(&self) -> &String {
		&self.cover
	}
	pub fn count(&self) -> i32 {
		self.videos
	}
	pub fn title(&self) -> &String {
		&self.title
	}
	pub fn private(&self) -> bool {
		self.private
	}
	pub fn privateEdit(&self) -> bool {
		self.privateEdit
	}
}

#[derive(Clone)]
pub struct Playlist {
	pub _id: ObjectId,
	pub item: PlaylistMeta,
	pub meta: Meta,
	pub clearence: i32,
	pub editable: Option<bool>,
	pub owner: Option<bool>,
	pub tags: Vec<i64>,
	pub tag_by_category: Option<Vec<TagCategoryItem>>,
	pub comment_thread: Option<ObjectId>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Playlist")]
impl Playlist {
	pub fn id(&self) -> ObjectId {
		self._id.clone()
	}
	pub fn clearence(&self) -> &i32 {
		&self.clearence
	}
	/// Metadata (created_at etc.)
	pub fn meta(&self) -> &Meta {
		&self.meta
	}
	/// Playlist metadata
	pub fn item(&self) -> &PlaylistMeta {
		&self.item
	}
	/// If current user can edit this playlist
	pub fn editable(&self) -> Option<bool> {
		self.editable
	}
	/// If current user can edit or delete this playlist
	pub fn owner(&self) -> Option<bool> {
		self.owner
	}
	pub async fn videos(&self, context: &Context, offset: Option<i32>, limit: Option<i32>) -> FieldResult<Vec<Video>> {
		let videos = playlist::getPlaylistContent_impl(context, playlist::GetPlaylistContentParameters {
			offset: offset,
			limit: limit,
			pid: self._id.to_string()
		}).await?;
		Ok(videos)
	}
	pub fn tag_ids(&self) -> Vec<i32> {
		self.tags.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
	}
	pub async fn tag_by_category(&self, context: &Context, lang: Option<String>) -> FieldResult<Vec<TagCategoryItem>> {
		if let Some(catemap) = self.tag_by_category.clone() {
			Ok(catemap)
		} else {
			//self.fill_missing_fields();
			let playlist_obj = playlist::getPlaylist_impl(context, playlist::GetPlaylistParameters {
				pid: self._id.to_string()
			}).await?;

			Ok(playlist_obj.tag_by_category.unwrap())
		}
	}
	pub async fn tags(&self, context: &Context) -> FieldResult<Vec<TagObjectValue>> {
		let tagobjs = editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: self.tags.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
		}).await?;
		let mut resp = vec![];
		for tagobj in tagobjs {
			let ret: TagObjectValue = if tagobj.category == "Author" {
				AuthorTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					author: match authorDB::getAuthor_impl(context, authorDB::GetAuthorParameters { tagid: tagobj.tagid }).await {
						Ok(ret) => Some(ret),
						Err(_) => None
					},
					is_author: true,
					author_role: "author".to_string(),
					meta: tagobj.meta
				}.into()
			} else {
				RegularTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					is_author: false,
					meta: tagobj.meta
				}.into()
			};
			resp.push(ret);
		};
		Ok(resp)
	}
	pub async fn rating(&self, context: &Context) -> FieldResult<Option<Rating>> {
		let rating = match rating::getRating_impl(context, rating::GetRatingParameters {
			pid: Some(self._id.to_string()),
			vid: None
		}).await {
			Ok(r) => r,
			Err(_) => None
		};
		Ok(rating)
	}
	pub async fn comment_thread(&self, context: &Context) -> FieldResult<Option<Thread>> {
		Ok(match self.comment_thread.as_ref() {
			Some(thread_id) => {
				Some(comment::getThread_impl(context, comment::GetThreadParameters {
					thread_id: thread_id.to_string()
				}).await?)
			},
			None => None
		})
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlaylistContentForVideo {
	pub _id: ObjectId,
	pub item: PlaylistMeta,
	pub rank: i32,
	pub next: Option<String>,
	pub prev: Option<String>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Playlist Content Generated From Query of a single video")]
impl PlaylistContentForVideo {
	pub fn id(&self) -> ObjectId {
		self._id.clone()
	}
	/// Playlist's metadata
	pub fn meta(&self) -> &PlaylistMeta {
		&self.item
	}
	/// Video's position in playlist
	pub fn rank(&self) -> i32 {
		self.rank
	}
	/// Get the actual playlist
	pub async fn playlist(&self, context: &Context) -> FieldResult<Playlist> {
		let playlist_meta = playlist::getPlaylist_impl(context, playlist::GetPlaylistParameters {
			pid: self._id.to_string()
		}).await?;
		Ok(playlist_meta)
	}
	pub async fn next(&self, context: &Context, lang: String) -> FieldResult<Option<Video>> {
		Ok(if let Some(vid) = &self.next {
			let vidobj = getVideo::getVideo_impl(context, getVideo::GetVideoParameters {
				lang: lang,
				vid: vid.to_string()
			}).await?;
			Some(vidobj)
		} else {
			None
		})
	}
	pub async fn prev(&self, context: &Context, lang: String) -> FieldResult<Option<Video>> {
		Ok(if let Some(vid) = &self.prev {
			let vidobj = getVideo::getVideo_impl(context, getVideo::GetVideoParameters {
				lang: lang,
				vid: vid.to_string()
			}).await?;
			Some(vidobj)
		} else {
			None
		})
	}
}

use crate::services::{authorDB, editTags, getVideo, playlist, rating, users};

#[derive(Clone, Serialize, Deserialize)]
pub struct Video {
	pub _id: ObjectId,
	pub clearence: i32,
	pub item: VideoItem,
	pub meta: Meta,
	pub tag_count: i32,
	pub tags: Vec<i64>,
	pub tags_readable: Option<Vec<String>>,
	pub tag_by_category: Option<Vec<TagCategoryItem>>,
	pub copies: Option<Vec<Video>>,
	pub playlists: Option<Vec<PlaylistContentForVideo>>,
	pub comment_thread: Option<ObjectId>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Video")]
impl Video {
	pub fn id(&self) -> ObjectId {
		self._id.clone()
	}
	pub fn clearence(&self) -> &i32 {
		&self.clearence
	}
	pub fn item(&self) -> &VideoItem {
		&self.item
	}
	pub fn meta(&self) -> &Meta {
		&self.meta
	}
	pub fn tag_count(&self) -> &i32 {
		&self.tag_count
	}
	pub fn tag_ids(&self) -> Vec<i32> {
		self.tags.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
	}
	pub fn tags_readable(&self) -> &Option<Vec<String>> {
		&self.tags_readable
	}
	pub async fn tag_by_category(&self, context: &Context, lang: String) -> FieldResult<Vec<TagCategoryItem>> {
		if let Some(catemap) = self.tag_by_category.clone() {
			Ok(catemap)
		} else {
			//self.fill_missing_fields();
			let vidobj = getVideo::getVideo_impl(context, getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;

			Ok(vidobj.tag_by_category.unwrap())
		}
	}
	pub async fn tags(&self, context: &Context) -> FieldResult<Vec<TagObjectValue>> {
		let tagobjs = editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: self.tags.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
		}).await?;
		let mut resp = vec![];
		for tagobj in tagobjs {
			let ret: TagObjectValue = if tagobj.category == "Author" {
				AuthorTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					author: match authorDB::getAuthor_impl(context, authorDB::GetAuthorParameters { tagid: tagobj.tagid }).await {
						Ok(ret) => Some(ret),
						Err(_) => None
					},
					is_author: true,
					author_role: "author".to_string(),
					meta: tagobj.meta
				}.into()
			} else {
				RegularTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					is_author: false,
					meta: tagobj.meta
				}.into()
			};
			resp.push(ret);
		};
		Ok(resp)
	}
	pub async fn copies(&self, context: &Context, lang: String) -> FieldResult<Vec<Video>> {
		if let Some(copies) = self.copies.clone() {
			Ok(copies)
		} else {
			let vidobj = getVideo::getVideo_impl(context, getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;
			Ok(vidobj.copies.unwrap())
		}
	}
	pub async fn playlists(&self, context: &Context, lang: String) -> FieldResult<Vec<PlaylistContentForVideo>> {
		if let Some(playlists) = self.playlists.clone() {
			Ok(playlists)
		} else {
			let vidobj = getVideo::getVideo_impl(context, getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;
			Ok(vidobj.playlists.unwrap())
		}
	}
	pub async fn rating(&self, context: &Context) -> FieldResult<Option<Rating>> {
		let rating = match rating::getRating_impl(context, rating::GetRatingParameters {
			vid: Some(self._id.to_string()),
			pid: None
		}).await {
			Ok(r) => r,
			Err(_) => None
		};
		Ok(rating)
	}
	pub async fn comment_thread(&self, context: &Context) -> FieldResult<Option<Thread>> {
		Ok(match self.comment_thread.as_ref() {
			Some(thread_id) => {
				Some(comment::getThread_impl(context, comment::GetThreadParameters {
					thread_id: thread_id.to_string()
				}).await?)
			},
			None => None
		})
	}
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="MultilingualMapping", Context = Context)]
pub struct MultilingualMapping {
	pub lang: String,
	pub value: String
}

#[derive(GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="RegularTagObject", impl = TagObjectValue, Context = Context)]
pub struct RegularTagObject {
	pub tagid: i32,
	pub _id: ObjectId,
	pub category: String,
	pub count: i32,
	pub languages: Vec<MultilingualMapping>,
	pub alias: Vec<String>,
	pub is_author: bool,
	pub meta: Meta
}

#[derive(GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="AuthorTagObject", impl = TagObjectValue, Context = Context)]
pub struct AuthorTagObject {
	pub tagid: i32,
	pub _id: ObjectId,
	pub category: String,
	pub count: i32,
	pub languages: Vec<MultilingualMapping>,
	pub alias: Vec<String>,
	pub author: Option<Author>,
	pub is_author: bool,
	pub meta: Meta,
	pub author_role: String
}

#[graphql_interface(for = [RegularTagObject, AuthorTagObject], Context = Context)] // enumerating all implementers is mandatory 
pub trait TagObject {
	async fn id(&self) -> ObjectId;
	async fn tagid(&self) -> i32;
	async fn category(&self) -> &String;
	async fn count(&self) -> i32;
	async fn languages(&self) -> &Vec<MultilingualMapping>;
	async fn alias(&self) -> &Vec<String>;
	async fn is_author(&self) -> bool;
	async fn meta(&self) -> &Meta;
}

#[juniper::graphql_interface]
impl TagObject for RegularTagObject {
	async fn id(&self) -> ObjectId {
		self._id.clone()
	}
	async fn tagid(&self) -> i32 {
		self.tagid
	}
	async fn category(&self) -> &String {
		&self.category
	}
	async fn count(&self) -> i32 {
		self.count
	}
	async fn languages(&self) -> &Vec<MultilingualMapping> {
		&self.languages
	}
	async fn alias(&self) -> &Vec<String> {
		&self.alias
	}
	async fn is_author(&self) -> bool {
		false
	}
	async fn meta(&self) -> &Meta {
		&self.meta
	}
}

#[juniper::graphql_interface]
impl TagObject for AuthorTagObject {
	async fn id(&self) -> ObjectId {
		self._id.clone()
	}
	async fn tagid(&self) -> i32 {
		self.tagid
	}
	async fn category(&self) -> &String {
		&self.category
	}
	async fn count(&self) -> i32 {
		self.count
	}
	async fn languages(&self) -> &Vec<MultilingualMapping> {
		&self.languages
	}
	async fn alias(&self) -> &Vec<String> {
		&self.alias
	}
	async fn is_author(&self) -> bool {
		true
	}
	async fn meta(&self) -> &Meta {
		&self.meta
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Author {
	pub _id: ObjectId,
	#[serde(rename = "type")]
	pub type_: String,
	#[serde(rename = "tagid")]
	pub tagname: String,
	pub common_tagids: Vec<i32>,
	pub urls: Vec<String>,
	pub user_space_ids: Vec<String>,
	pub avatar: String,
	pub desc: String
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Author")]
impl Author {
	pub async fn id(&self) -> ObjectId {
		self._id.clone()
	}
	#[graphql(
        // overwrite the public name
        name = "type"
    )]
	pub async fn type_(&self) -> &String {
		&self.type_
	}
	pub async fn tagname(&self) -> &String {
		&self.tagname
	}
	pub async fn common_tagids(&self) -> &Vec<i32> {
		&self.common_tagids
	}
	pub async fn common_tags(&self, context: &Context) -> FieldResult<Vec<TagObjectValue>> {
		let tagobjs = editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: self.common_tagids.clone()
		}).await?;
		let mut resp = vec![];
		for tagobj in tagobjs {
			let ret: TagObjectValue = if tagobj.category == "Author" {
				AuthorTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					author: match authorDB::getAuthor_impl(context, authorDB::GetAuthorParameters { tagid: tagobj.tagid }).await {
						Ok(ret) => Some(ret),
						Err(_) => None
					},
					is_author: true,
					author_role: "author".to_string(),
					meta: tagobj.meta
				}.into()
			} else {
				RegularTagObject {
					tagid: tagobj.tagid,
					_id: tagobj._id.clone(),
					alias: tagobj.alias.clone(),
					category: tagobj.category.clone(),
					languages: tagobj.languages.clone(),
					count: tagobj.count,
					is_author: false,
					meta: tagobj.meta
				}.into()
			};
			resp.push(ret);
		};
		Ok(resp)
	}
	pub async fn urls(&self) -> &Vec<String> {
		&self.urls
	}
	pub async fn user_space_ids(&self) -> &Vec<String> {
		&self.user_space_ids
	}
	pub async fn avatar(&self) -> &String {
		&self.avatar
	}
	pub async fn desc(&self) -> &String {
		&self.desc
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Subscription {
	pub _id: ObjectId,
	/// Query
	pub qs: String,
	/// Query type, one of 'tag', 'text'
	pub qt: String,
	/// Name of this subscription
	pub name: Option<String>,
	pub meta: Meta
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Subscription")]
impl Subscription {
	pub async fn id(&self) -> ObjectId {
		self._id.clone()
	}
	/// Query
	pub fn query(&self) -> &String {
		&self.qs
	}
	/// Query type, one of 'tag', 'text'
	pub fn query_type(&self) -> &String {
		&self.qt
	}
	/// Name of this query
	pub fn name(&self) -> Option<String> {
		match self.name.as_ref() {
			Some(s) => {
				if s.len() > 0 {
					Some(s.clone())
				} else {
					None
				}
			},
			None => None
		}
	}
	pub fn meta(&self) -> &Meta {
		&self.meta
	}
}

