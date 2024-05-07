
use juniper::graphql_value;


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
	pub category: TagCategoryEnum,
	pub count: f64,
	pub languages: serde_json::Map<String, serde_json::Value>,
	pub alias: Vec<String>,
	pub meta: Meta
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagObjectResp {
	pub tag_objs: Vec<TagObjectRespObject>
}

pub async fn getTagObjectsBatch_impl(context: &Context, para: GetTagObjectsBatchParameters) -> FieldResult<Vec<TagObjectValue>> {
	let result = postJSON!(TagObjectResp, format!("{}/tags/get_tag_batch.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let tagobjs = result.data.unwrap().tag_objs;
		let mut resp = vec![];
		for tagobj in tagobjs {
			let ret: TagObjectValue = if tagobj.category == TagCategoryEnum::Author {
				AuthorTagObject {
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
					author: match authorDB::getAuthor_impl(context, authorDB::GetAuthorParameters { tagid: tagobj.id }).await {
						Ok(ret) => Some(ret),
						Err(_) => None
					},
					is_author: true,
					author_role: "author".to_string(),
					meta: tagobj.meta
				}.into()
			} else {
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
					meta: tagobj.meta
				}.into()
			};
			resp.push(ret);
		};
		Ok(resp)
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

pub async fn getTagObjectsBatchRegular_impl(context: &Context, para: GetTagObjectsBatchParameters) -> FieldResult<Vec<RegularTagObject>> {
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
			let ret: TagObjectValue = if tagobj.category == TagCategoryEnum::Author {
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
			tags: ret.tags.into_iter().map(|tagobj| {
					RegularTagObject {
						tagid: tagobj.id,
						_id: tagobj._id,
						alias: tagobj.alias,
						category: tagobj.category,
						languages: {
							let mut langmap: Vec<MultilingualMapping> = vec![];
							for (k, v) in tagobj.languages {
								langmap.push(MultilingualMapping {
									lang: k,
									value: v.as_str().unwrap().to_string()
								});
							};
							langmap
						},
						count: tagobj.count as i32,
						is_author: false,
						meta: tagobj.meta
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


#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize, Debug)]
#[graphql(description="required parameters for adding a tag", Context = Context)]
pub struct AddTagParameters {
	/// Tag
	pub tag: String,
	/// Category
	pub category: String,
	/// Language
	pub language: String,
}

pub async fn addTag_impl(context: &Context, para: AddTagParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/add_tag.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for removing a tag", Context = Context)]
pub struct RemoveTagParameters {
	/// Tag
	pub tag: String,
}

pub async fn removeTag_impl(context: &Context, para: RemoveTagParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/remove_tag.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for transfer tag category", Context = Context)]
pub struct TransferCategoryParameters {
	/// Tag
	pub tag: String,
	/// Category
	pub category: String,
}

pub async fn transferCategory_impl(context: &Context, para: TransferCategoryParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/transfer_category.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for renaming a tag", Context = Context)]
pub struct RenameTagParameters {
	/// Tag
	pub tag: String,
	/// New Tag
	pub new_tag: String,
	/// Language
	pub language: String,
}

pub async fn renameTag_impl(context: &Context, para: RenameTagParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/rename_tag.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for renaming an alias", Context = Context)]
pub struct RenameAliasParameters {
	/// Tag
	pub tag: String,
	/// New Tag
	pub new_tag: String,
}

pub async fn renameAlias_impl(context: &Context, para: RenameAliasParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/rename_alias.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for adding an alias", Context = Context)]
pub struct AddAliasParameters {
	/// Tag
	pub tag: String,
	/// New Tag
	pub new_tag: String,
}

pub async fn addAlias_impl(context: &Context, para: AddAliasParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/add_alias.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for adding a tag language", Context = Context)]
pub struct AddTagLanguageParameters {
	/// Tag
	pub tag: String,
	/// New Tag
	pub new_tag: String,
	/// Language
	pub language: String,
}

pub async fn addTagLanguage_impl(context: &Context, para: AddTagLanguageParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/add_tag_language.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for removing an alias", Context = Context)]
pub struct RemoveAliasParameters {
	/// Alias
	pub alias: String,
}

pub async fn removeAlias_impl(context: &Context, para: RemoveAliasParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/remove_alias.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for merging tags", Context = Context)]
pub struct MergeTagParameters {
	/// Tag dst
	pub tag_dst: String,
	/// Tag src
	pub tag_src: String,
}

pub async fn mergeTag_impl(context: &Context, para: MergeTagParameters) -> FieldResult<bool> {
	let result = postJSON!(EmptyJSON, format!("{}/tags/merge_tag.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(true)
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
