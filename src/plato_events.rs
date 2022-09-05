use serde_derive::{Serialize, Deserialize};
use serde::Serialize;
use zotero::data_structure::ToJson;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PlatoEvent {
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

	#[serde(rename="searchresult")]
	SearchResults {
		result: String,
	}
}

impl ToJson for PlatoEvent {}

impl PlatoEvent {
	pub fn message(msg: &dyn ToString) -> Self {
		Self::Notification {
			message: msg.to_string()
		}
	}
	pub fn enableWifi() -> Self {
		Self::Wifi {
			enabled: true
		}
	}
	pub fn disableWifi() -> Self {
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
}
