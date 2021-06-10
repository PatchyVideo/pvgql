
use juniper::{ScalarValue, graphql_value};


use juniper::FieldResult;

use crate::{common::*, models::{TagObject, TagCategoryItem, RegularTagObject, AuthorTagObject}};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="GetPopularTags parameters", Context = Context)]
pub struct GetPopularTagsParameters {
	/// Language, default 'ENG'
    pub lang: Option<String>,
    // How many tags, default 20
    pub count: Option<i32>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPopularTagsResult {
	pub tagids_popmap: Option<serde_json::Map<String, serde_json::Value>>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="GetPopularTags result")]
impl GetPopularTagsResult {
	pub async fn popular_tags(&self, context: &Context) -> FieldResult<Option<Vec<TagWithPopularity>>> {
		if let Some(tagid_maps) = self.tagids_popmap.as_ref() {
			let tagids = tagid_maps.keys().map(|k| k.parse::<i64>().unwrap()).collect::<Vec<_>>();
			let mut tagobjs: Vec<TagObjectValue> = super::editTags::getTagObjectsBatch_impl(context, super::editTags::GetTagObjectsBatchParameters {
				tagid: tagids.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
			}).await?;
			Ok(Some(tagid_maps.values().zip(tagobjs.iter_mut()).map(|(k, v)| {
				TagWithPopularity {
					popluarity: k.as_i64().unwrap() as _,
					tag: v.clone()
				}
			}).collect::<Vec<_>>()))
		} else {
			Ok(None)
		}
	}
}

pub async fn getPopularTags_impl(context: &Context, para: GetPopularTagsParameters) -> FieldResult<GetPopularTagsResult> {
	let result = postJSON!(GetPopularTagsResult, format!("{}/tags/popular_tags.do", BACKEND_URL), para, context);
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
