
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::{FieldResult, ScalarValue};

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;
use crate::services::authorDB;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="getTagsBatch required parameters", Context = Context)]
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
	pub alias: Vec<String>,
	pub meta: Meta
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagObjectResp {
	pub tag_objs: Vec<TagObjectRespObject>
}

pub async fn getTagObjectsBatch_impl(context: &Context, para: GetTagObjectsBatchParameters) -> FieldResult<Vec<RegularTagObject>> {
	let result = postJSON!(TagObjectResp, format!("{}/tags/get_tag_batch.do", BACKEND_URL), para, context);
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
				is_author: false,
				meta: tagobj.meta.clone()
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

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize, Debug)]
#[graphql(description="required parameters for listing tags", Context = Context)]
pub struct ListTagParameters {
	/// Query
	pub query: Option<String>,
	/// Use regex for query if exists and true, otherwise wildcard query will be used
	pub query_regex: Option<bool>,
    /// Category
    pub category: Option<String>,
    /// Order, one of 'latest', 'oldest', 'count', 'count_inv'
	pub order: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagsRespObject {
	pub tags: Vec<TagObjectRespObject>,
	pub count: i32,
	pub page_count: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListTagsResult {
	pub tags: Vec<RegularTagObject>,
	pub count: i32,
	pub page_count: i32
}

#[juniper::graphql_object(Context = Context)]
#[juniper::graphql(description="List tags result")]
impl ListTagsResult {
	pub async fn tags(&self, context: &Context) -> Vec<TagObjectValue> {
		let mut result = Vec::new();
		for tagobj in self.tags.iter() {
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
					meta: tagobj.meta.clone()
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
					meta: tagobj.meta.clone()
				}.into()
			};
			result.push(ret);
		}
		result
	}
	pub fn count(&self) -> &i32 {
		&self.count
	}
	pub fn page_count(&self) -> &i32 {
		&self.page_count
	}
}

pub async fn listTags_impl(context: &Context, para: ListTagParameters) -> FieldResult<ListTagsResult>
{
	let mut result_opt = None;
	if para.query.is_none() && para.category.is_some() {
		result_opt = Some(postJSON!(ListTagsRespObject, format!("{}/tags/query_tags.do", BACKEND_URL), para, context));
	} else if para.query.is_some() {
		let use_regex = para.query_regex.map_or(false, |f| f);
		if use_regex {
			result_opt = Some(postJSON!(ListTagsRespObject, format!("{}/tags/query_tags_regex.do", BACKEND_URL), para, context));
		} else {
			result_opt = Some(postJSON!(ListTagsRespObject, format!("{}/tags/query_tags_wildcard.do", BACKEND_URL), para, context));
		}
	};
	if result_opt.is_none() {
        return Err(
            juniper::FieldError::new(
                "INCORRECT_REQUEST",
                graphql_value!({
                    "At least one of query or category must be set"
                }),
            )
        );
	};
	let result = result_opt.unwrap();
	if result.status == "SUCCEED" {
		let ret = result.data.unwrap();
		Ok(ListTagsResult {
			tags: ret.tags.iter().map(|tagobj| {
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
						is_author: false,
						meta: tagobj.meta.clone()
					}
				}).collect::<Vec<_>>(),
			page_count: ret.page_count,
			count: ret.count
			}
		)
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

