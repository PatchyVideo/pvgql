
macro_rules! postRawJSON {
	($t:ident , $u:expr, $j:expr) => {
		{
			let client = reqwest::blocking::Client::new();
			let response = client.post(&$u).json(&$j).send()?;
			if response.status().is_success() {
				println!("resp body: {:?}", response);
				let resp_str = response.text().unwrap();
				println!("body: {}", resp_str);
				let obj : $t = serde_json::from_str(resp_str.as_str())?; //response.json()?;
				Ok(obj)
			} else {
				let e: Error = response.json()?;
				Err(
					juniper::FieldError::new(
						e.code,
						graphql_value!({
							e.aux
						}),
					)
				)
			}
		}?
	};
}

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct EmptyJSON {
	/// Always not present
	pub empty: Option<i32>
}
impl EmptyJSON {
	pub fn new() -> EmptyJSON {
		EmptyJSON {
			empty: None
		}
	}
}

#[cfg(debug_assertions)]
pub const BACKEND_URL: &str = "https://patchyvideo.com/be";

#[cfg(not(debug_assertions))]
pub const BACKEND_URL: &str = "http://patchyvideo-primary-stack_web:5000";

macro_rules! postJSON {
	($t:ident, $u:expr, $j:expr, $c:ident) => {
		{
			let client = reqwest::Client::new();
			let client = client.post(&$u);
			let client = match $c.session.as_ref() {
				Some(sess) => client.header("cookie", format!("session={}", sess)),
				None => client
			};
			let client = match $c.auth_header.as_ref() {
				Some(auth) => client.header("Authorization", auth),
				None => client
			};
			let response = client.json(&$j).send().await?;
			if response.status().is_success() {
				let obj : RestResult::<$t> = response.json().await?;
				Ok(obj)
			} else {
				let e: Error = response.json().await?;
				Err(
					juniper::FieldError::new(
						e.code,
						graphql_value!({
							e.aux
						}),
					)
				)
			}
		}?
	};
}

macro_rules! postJSON_empty {
	($u:expr, $j:expr, $c:ident) => {
		{
			let client = reqwest::Client::new();
			let response = match $c.session.as_ref() {
				Some(sess) => client.post(&$u).header("cookie", format!("session={}", sess)).json(&$j).send().await?,
				None => client.post(&$u).json(&$j).send().await?
			};
			if response.status().is_success() {
				response.json().await?;
				Ok(())
			} else {
				let e: Error = response.json().await?;
				Err(
					juniper::FieldError::new(
						e.code,
						graphql_value!({
							e.aux
						}),
					)
				)
			}
		}?
	};
}
