use crate::{Error, Result, web};
use serde::Deserialize;
use axum::{Json, Router, routing::post};
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

pub fn routes()-> Router {
	Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies:Cookies,payload:Json<LoginPayload>) -> Result<Json<Value>>{
	println!("->> {:<12} - api_login", "HANDLER");
	if payload.email != "test" || payload.password != "test" {
		return Err(Error::LoginFail)
	}

	cookies.add(Cookie::new(web::AUTH_TOKEN,"user-1.exp.sign"));

	let body = Json (json!({
		"result":{
			"success":true,
			"message":"Login success"
		}
	}));
	Ok(body)
}
#[derive(Debug, Deserialize)]
struct LoginPayload {
	email:String,
	password: String,
}