use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Recipe{
	pub id: u32,
	pub title: String,
	pub ingredients: Vec<String>,
	pub instructions: String,
}
