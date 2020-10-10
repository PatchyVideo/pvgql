
extern crate serde_json;

use juniper::{
    execute,
    parser::{ParseError, ScalarToken, Spanning, Token},
    serde::de,
    EmptyMutation, FieldResult, InputValue, Object, ParseScalarResult, RootNode, ScalarValue,
    Value, Variables,
};
use std::fmt;

use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Meta {
	pub created_at: BsonDateTime,
	pub created_by: ObjectId,
	pub modified_at: Option<BsonDateTime>,
	pub modified_by: Option<ObjectId>
}

#[juniper::graphql_object]
#[graphql(description="Meta")]
impl Meta {
	pub fn nmsl(&self) -> String {
		"nmsl".into()
	}
}


// #[derive(Serialize, Deserialize)]
// pub struct VideoItemRest {
//     pub cover_image: String,
//     pub title: String,
//     pub desc: String,
//     pub placeholder: bool,
//     pub rating: i32,
//     pub repost_type: String,
//     pub copies: Vec<ObjectId>,
//     pub series: Vec<ObjectId>,
//     pub site: String,
//     pub thumbnail_url: String,
//     pub unique_id: String,
//     pub upload_time: DateTime,
//     pub url: String,
//     pub user_space_urls: Option<Vec<String>>,
//     pub utags: Vec<String>,
//     pub views: i32
// }

// #[derive(Serialize, Deserialize)]
// pub struct VideoRest {
//     pub clearence: i32,
//     pub item: VideoItemRest,
//     pub meta: MetaRest,
//     pub tag_count: i32,
//     pub tags: Vec<i32>,
//     pub tags_readable: Option<Vec<String>>
// }

#[derive(Clone, Serialize, Deserialize)]
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
	pub placeholder: bool,
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

#[juniper::graphql_object]
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
	pub fn placeholder(&self) -> &bool {
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
	pub playlists: Option<Vec<PlaylistContentForVideo>>
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

#[juniper::graphql_object]
#[graphql(description="PlaylistMeta")]
impl PlaylistMeta {
	pub fn cover(&self) -> &String {
		&self.cover
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlaylistContentForVideo {
	pub _id: ObjectId,
	pub item: PlaylistMeta,
	pub next: Option<String>,
	pub prev: Option<String>
}

#[juniper::graphql_object]
#[graphql(description="Playlist Content Generated From Query of a single video")]
impl PlaylistContentForVideo {
	pub fn id(&self) -> String {
		self._id.to_string()
	}
	pub fn item(&self) -> &PlaylistMeta {
		&self.item
	}
	pub async fn next(&self, lang: String) -> FieldResult<Option<Video>> {
		Ok(if let Some(vid) = &self.next {
			let vidobj = getVideo::getVideo_impl(getVideo::GetVideoParameters {
				lang: lang,
				vid: vid.to_string()
			}).await?;
			Some(vidobj)
		} else {
			None
		})
	}
	pub async fn prev(&self, lang: String) -> FieldResult<Option<Video>> {
		Ok(if let Some(vid) = &self.prev {
			let vidobj = getVideo::getVideo_impl(getVideo::GetVideoParameters {
				lang: lang,
				vid: vid.to_string()
			}).await?;
			Some(vidobj)
		} else {
			None
		})
	}
}

// impl Video {
// 	pub fn fill_missing_fields(&mut self) {

// 	}
// }

use crate::services::getVideo;

#[juniper::graphql_object]
#[graphql(description="Video")]
impl Video {
	pub fn id(&self) -> String {
		self._id.to_string()
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
	pub fn tags(&self) -> Vec<i32> {
		self.tags.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
	}
	pub fn tags_readable(&self) -> &Option<Vec<String>> {
		&self.tags_readable
	}
	pub async fn tag_by_category(&self, lang: String) -> FieldResult<Vec<TagCategoryItem>> {
		if let Some(catemap) = self.tag_by_category.clone() {
			Ok(catemap)
		} else {
			//self.fill_missing_fields();
			let vidobj = getVideo::getVideo_impl(getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;

			Ok(vidobj.tag_by_category.unwrap())
		}
	}
	pub async fn copies(&self, lang: String) -> FieldResult<Vec<Video>> {
	    if let Some(copies) = self.copies.clone() {
			Ok(copies)
		} else {
			let vidobj = getVideo::getVideo_impl(getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;
			Ok(vidobj.copies.unwrap())
		}
	}
	pub async fn playlists(&self, lang: String) -> FieldResult<Vec<PlaylistContentForVideo>> {
	    if let Some(playlists) = self.playlists.clone() {
			Ok(playlists)
		} else {
			let vidobj = getVideo::getVideo_impl(getVideo::GetVideoParameters {
				lang: lang,
				vid: self._id.to_string()
			}).await?;
			Ok(vidobj.playlists.unwrap())
		}
	}
}
