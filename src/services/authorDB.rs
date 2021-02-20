
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
use crate::services::editTags;

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
					author: match getAuthor_impl(context, GetAuthorParameters { tagid: tagobj.tagid }).await {
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


#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="getTagsBatch required parameters", Context = Context)]
pub struct GetAuthorParameters {
	/// Tag ID
    pub tagid: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAuthorResp {
    pub record: Author
}

pub async fn getAuthor_impl(context: &Context, para: GetAuthorParameters) -> FieldResult<Author> {
	let result = postJSON!(GetAuthorResp, format!("{}/authors/get_record_raw.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(result.data.unwrap().record)
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
