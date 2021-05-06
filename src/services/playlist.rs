
use juniper::graphql_value;


use juniper::FieldResult;

use crate::{common::*, models::{Playlist, TagCategoryEnum, TagCategoryItem}};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, PlaylistMeta};
use crate::context::Context;

#[derive(Clone, Serialize, Deserialize)]
pub struct ResultantPlaylist {
	pub _id: ObjectId,
    pub item: PlaylistMeta,
    pub meta: Meta,
    pub tag_count: i32,
    pub tags: Vec<i64>,
	pub clearence: i32,
	pub comment_thread: Option<ObjectId>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPlaylistMetadataResult {
    pub editable: bool,
    pub owner: bool,
    pub playlist: ResultantPlaylist,
    pub tags: Vec<serde_json::Value>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPlaylistContentResult {
    pub videos: Vec<Video>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get playlist", Context = Context)]
pub struct GetPlaylistParameters {
	/// ID of playlist
    pub pid: String
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get playlist content", Context = Context)]
pub struct GetPlaylistContentParameters {
    /// ID of playlist
    pub pid: String,
	/// Offset (start from 0)
	pub offset: Option<i32>,
	/// Num of item in a page
	pub limit: Option<i32>,
}

/// Only loads metadata
pub async fn getPlaylist_impl(context: &Context, para: GetPlaylistParameters) -> FieldResult<Playlist> {
    let result = postJSON!(GetPlaylistMetadataResult, format!("{}/lists/get_playlist_metadata.do", BACKEND_URL), para, context);
    if result.status == "SUCCEED" {
        let r = result.data.unwrap();
        let tag_by_cat = r.tags[2].as_object().ok_or(juniper::FieldError::new(
            "NO_CATEGORY_TAG_MAP",
            graphql_value!({
                "INTERAL_ERROR": "NO_CATEGORY_TAG_MAP"
            }),
        ))?;
        let mut catemap: Vec<TagCategoryItem> = vec![];
		for (k, v) in tag_by_cat {
			catemap.push(TagCategoryItem {
				key: TagCategoryEnum::from_string(&k)?,
				value: v.as_array().unwrap().iter().map(|x: &serde_json::Value| x.as_str().unwrap().into()).collect::<Vec<_>>()
			});
		};
		Ok(Playlist {
            _id: r.playlist._id,
            item: r.playlist.item,
            meta: r.playlist.meta,
            clearence: r.playlist.clearence,
            editable: Some(r.editable),
            owner: Some(r.owner),
            tags: r.playlist.tags,
            tag_by_category: Some(catemap),
			comment_thread: r.playlist.comment_thread
        })
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}

pub async fn getPlaylistContent_impl(context: &Context, para: GetPlaylistContentParameters) -> FieldResult<Vec<Video>> {
    let result = postJSON!(GetPlaylistContentResult, format!("{}/lists/get_playlist.do", BACKEND_URL), para, context);
    if result.status == "SUCCEED" {
        let r = result.data.unwrap();
		Ok(r.videos)
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listPlaylist required parameters", Context = Context)]
pub struct ListPlaylistParameters {
	/// Offset (start from 0)
	pub offset: Option<i32>,
	/// Num of item in a page
	pub limit: Option<i32>,
	/// Query
	pub query: Option<String>,
	/// List order, one of 'latest', 'oldest', 'last_modified'
	pub order: Option<String>,
	// Addtional query constraints
	pub additional_constraint: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListPlaylistResult {
	pub playlists: Vec<ResultantPlaylist>,
	pub count: i32,
	pub page_count: i32
}

#[juniper::graphql_object(Context = Context)]
#[juniper::graphql(description="List playlist result")]
impl ListPlaylistResult {
	pub fn playlists(&self) -> Vec<Playlist> {
		self.playlists.iter().map(|r| Playlist {
            _id: r._id.clone(),
            item: r.item.clone(),
            meta: r.meta.clone(),
            clearence: r.clearence,
            editable: None,
            owner: None,
            tags: r.tags.clone(),
            tag_by_category: None,
			comment_thread: r.comment_thread.clone()
        }).collect::<Vec<_>>()
	}
	pub fn count(&self) -> &i32 {
		&self.count
	}
	pub fn page_count(&self) -> &i32 {
		&self.page_count
	}
}

pub async fn listPlatylist_impl(context: &Context, para: ListPlaylistParameters) -> FieldResult<ListPlaylistResult> {
	let result = if para.query.is_none() {
		postJSON!(ListPlaylistResult, format!("{}/lists/all.do", BACKEND_URL), para, context)
	} else {
		postJSON!(ListPlaylistResult, format!("{}/lists/search.do", BACKEND_URL), para, context)
	};
	if result.status == "SUCCEED" {
		Ok(result.data.unwrap())
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}

