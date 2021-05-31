
use juniper::graphql_value;


use juniper::{FieldResult, ScalarValue};

use crate::common::*;
use crate::services::authorDB;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;
use super::editTags;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="editVideoTags required parameters", Context = Context)]
pub struct EditVideoTagsParameters {
	/// Video ID
    pub video_id: String,
    /// Tags
    pub tags: Vec<String>,
    /// One of 'replace', 'append', 'remove'
    pub edit_behaviour: String,
    /// Behaviour if a tag does not exist, one of 'ignore', 'error', 'append', default 'ignore'
    pub not_found_behaviour: Option<String>,
    /// User language used for adding tags, default is 'ENG'
    pub user_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditVideoTagsRespObject {
	pub tagids: Vec<i32>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="hideVideo required parameters", Context = Context)]
pub struct HideVideoParameters {
	/// Video ID
    pub video_id: String,
    /// Clearence, one of 0, 1, 2, 3, default is 0
    pub clearence: Option<i32>
}

pub async fn editVideoTags(context: &Context, para: EditVideoTagsParameters) -> FieldResult<Vec<TagObjectValue>> {
    let result = postJSON!(EditVideoTagsRespObject, format!("{}/videos/edittags.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
        let tagids = result.data.unwrap().tagids;
        let tagobjs = editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: tagids
		}).await?;
		let mut resp = vec![];
		for tagobj in tagobjs {
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


pub async fn hideVideo(context: &Context, para: HideVideoParameters) -> FieldResult<i32> {
    todo!()
}

