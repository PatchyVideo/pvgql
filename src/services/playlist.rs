
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::{common::*, models::{Playlist, TagCategoryItem}};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, PlaylistMeta};

#[derive(Clone, Serialize, Deserialize)]
pub struct ResultantPlaylist {
	pub _id: ObjectId,
    pub item: PlaylistMeta,
    pub meta: Meta,
    pub tag_count: i32,
    pub tags: Vec<i64>,
	pub clearence: i32,
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
#[graphql(description="required parameters for get playlist")]
pub struct GetPlaylistParameters {
	/// ID of playlist
    pub pid: String
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get playlist content")]
pub struct GetPlaylistContentParameters {
    /// ID of playlist
    pub pid: String,
	/// Offset (start from 0)
	pub offset: i32,
	/// Num of item in a page
	pub limit: i32,
}

/// Only loads metadata
pub async fn getPlaylist_impl(para: GetPlaylistParameters) -> FieldResult<Playlist> {
    let result = postJSON!(GetPlaylistMetadataResult, format!("https://thvideo.tv/be/lists/get_playlist_metadata.do"), para);
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
				key: k.clone(),
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
            tag_by_category: Some(catemap)
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

pub async fn getPlaylistContent_impl(para: GetPlaylistContentParameters) -> FieldResult<Vec<Video>> {
    let result = postJSON!(GetPlaylistContentResult, format!("https://thvideo.tv/be/lists/get_playlist.do"), para);
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
#[graphql(description="listPlaylist required parameters")]
pub struct ListPlaylistParameters {
	/// Offset (start from 0)
	pub offset: i32,
	/// Num of item in a page
	pub limit: i32,
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

#[juniper::graphql_object]
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
            tag_by_category: None
        }).collect::<Vec<_>>()
	}
	pub fn count(&self) -> &i32 {
		&self.count
	}
	pub fn page_count(&self) -> &i32 {
		&self.page_count
	}
}

pub async fn listPlatylist_impl(para: ListPlaylistParameters) -> FieldResult<ListPlaylistResult> {
	let result = if para.query.is_none() {
		postJSON!(ListPlaylistResult, format!("https://thvideo.tv/be/lists/all.do"), para)
	} else {
		postJSON!(ListPlaylistResult, format!("https://thvideo.tv/be/lists/search.do"), para)
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

