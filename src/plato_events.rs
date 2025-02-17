use std::path::Path;

use chrono::{DateTime, Datelike};
use serde::{Serialize, Deserialize};
use zotero_data::{ToJson, item::Item};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatoFileInfo {
	path: String,
	kind: String,
	size: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatoDocumentInfo {
	title: String,
	author: String,
	year: i32,
	identifier: String,
	added: String,
	file: PlatoFileInfo
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PlatoMessage {
	#[serde(rename="notify")]
	Notification {
		message: String
	},

	#[serde(rename="setWifi")]
	Wifi {
		enabled: bool
	},

	#[serde(rename="search")]
	Search {
		path: String,
		query: String,
		sortBy: (String, bool)
	},

	#[serde(rename="addDocument")]
	AddDocument {
		info: PlatoDocumentInfo
	}
}

impl ToJson for PlatoMessage {}

impl PlatoMessage {
	pub fn notify(msg: &dyn ToString) -> Self {
		Self::Notification {
			message: msg.to_string()
		}
	}
	pub fn enable_wifi() -> Self {
		Self::Wifi {
			enabled: true
		}
	}
	pub fn disable_wifi() -> Self {
		Self::Wifi {
			enabled: false
		}
	}
	pub fn serach(path: &dyn ToString, query: &dyn ToString, sorting: &dyn ToString, reverse: bool) -> Self {
		Self::Search {
			path: path.to_string(),
			query: query.to_string(),
			sortBy: ( sorting.to_string(), reverse )
		}
	}

	pub fn add_document(path: &Path, document_info: &Item) -> Self {

		// Get the file extension
		let extension = path.extension().unwrap().to_str().unwrap();

		// Get the file size in bytes
		let metadata = std::fs::metadata(path).unwrap();
		let size = metadata.len();

		let file_info = PlatoFileInfo{
			path: path.to_str().unwrap().to_string(),
			kind: extension.to_string(),
			size: size
		};


		Self::AddDocument {
			info: PlatoDocumentInfo {
				title: document_info.title().to_string(),
				author: document_info.author(),
				year: document_info.date().year(),
				identifier: Item::key(&document_info).to_string(),
				added: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
				file: file_info
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PlatoResponse {
	#[serde(rename="search")]
	SearchResults {
		results: Vec<Value>,
	}
}
