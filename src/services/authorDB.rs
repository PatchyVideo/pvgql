
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
pub struct GetAuthorParameters {
	/// Tag ID
    pub tagid: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAuthorResp {
    pub record: Author
}

pub async fn getAuthor_impl(para: GetAuthorParameters) -> FieldResult<Author> {
	let result = postJSON!(GetAuthorResp, format!("https://thvideo.tv/be/authors/get_record_raw.do"), para);
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
