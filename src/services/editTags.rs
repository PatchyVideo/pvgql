
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::{FieldResult, ScalarValue};

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="getTagsBatch required parameters")]
pub struct GetTagObjectsBatchParameters {
	/// Tag IDs
	pub tagid: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagObjectRespObject {
	pub id: i32,
	pub _id: ObjectId,
	pub category: String,
	pub count: f64,
	pub languages: serde_json::Map<String, serde_json::Value>,
	pub alias: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagObjectResp {
	pub tag_objs: Vec<TagObjectRespObject>
}

pub async fn getTagObjectsBatch_impl(para: GetTagObjectsBatchParameters) -> FieldResult<Vec<RegularTagObject>> {
	let result = postJSON!(TagObjectResp, format!("https://thvideo.tv/be/tags/get_tag_batch.do"), para);
	if result.status == "SUCCEED" {
		Ok(result.data.unwrap().tag_objs.iter().map(|tagobj| {
			RegularTagObject {
				tagid: tagobj.id,
				_id: tagobj._id.clone(),
				alias: tagobj.alias.clone(),
				category: tagobj.category.clone(),
				languages: {
					let mut langmap: Vec<MultilingualMapping> = vec![];
					for (k, v) in tagobj.languages.clone() {
						langmap.push(MultilingualMapping {
							lang: k,
							value: v.as_str().unwrap().to_string()
						});
					};
					langmap
				},
				count: tagobj.count as i32,
				is_author: false
			}
		}).collect::<Vec<_>>())
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
