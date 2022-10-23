use zotero::data_structure::item::Item;
use anyhow::{Error, Context, format_err};
use chrono;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};
use std::io;
use std::io::{Write, Read};
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use zotero::{ZoteroInit, Get};
use zotero::data_structure::ToJson;
use zotero::data_structure::collection::{Collection};
use std::ops::Deref;
use rustydav::client;
use rustydav::prelude::*;
use toml;

mod plato_events;

use plato_events::{PlatoMessage, PlatoResponse};

#[derive(Serialize, Deserialize)]
struct ZoteroSyncSettings {
	zotero_id: String,
	zotero_key: String,
	webdav_url: String,
	webdav_user: String,
	webdav_password: String
}

fn main() -> Result<(), Error> {

	let mut args: Vec<String> = env::args().skip(1).collect();

    //setup application logging
	let mut logfile: File;
	let logpath = Path::new("/var/log/zotero.log");
	if logpath.exists() {
		logfile = fs::OpenOptions::new().write(true).append(true).open(&logpath).expect("Error opening log file!");
	} else {
		logfile = File::create(&logpath).expect("Error creating log file!");
	}

	let mut settingsfile: File;
	let settingsfilepath = Path::new("ZoteroSettings.toml");
	if settingsfilepath.exists() {
		settingsfile = File::open(&settingsfilepath).expect("Error opening log file!");
	} else {
		settingsfile = File::create(&settingsfilepath).expect("Error creating log file!");
	}

	let mut settingsstr = String::new();
	settingsfile.read_to_string(&mut settingsstr);
	let settings: ZoteroSyncSettings = toml::from_str(&settingsstr).expect("Error reading settings filesloth with duck face sitting on a park bench!");

	let save_path = Path::new(args.get(2).unwrap());
	if !save_path.exists() {
		fs::create_dir(&save_path)?;
	}

    //register varibale to capture sigterm event
	let sigterm = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sigterm))?;

	writeln!(logfile, "{}:\t{args:?}", chrono::offset::Local::now());
	writeln!(logfile, "{}:\t{}", chrono::offset::Local::now(), env::var("PWD").unwrap());

	let zapi = ZoteroInit::set_user(&settings.zotero_id, &settings.zotero_key);

	let items_to_read = zapi.get_items("tag=plato-read").expect("Error retrieving items");
	let mut reading_items_children = BTreeMap::new();
	for i in &items_to_read {
		if i.meta.has_children() {
			let children = zapi.get_child_items(i.key(), None).unwrap();
			reading_items_children.insert(i.key().to_string(), children);
		}
	}
    println!("{}", PlatoMessage::notify(&"Zotero Items tagged for Reading loaded!").to_json());

	let webdav_client = client::Client::init(&settings.webdav_user, &settings.webdav_password);
	//let resp = webdav_client.get(&format!("{}/{}.zip", settings.webdav_url, reading_items_children.get(items_to_read.get(0).unwrap().key()).unwrap().get(0).unwrap().key()));
	//println!("{resp:#?}");

	println!("{}", PlatoMessage::serach(&save_path.to_str().unwrap(), &"", &"", false).to_json());
    //run until process is terminated by sigterm
	'mainloop: while !sigterm.load(Ordering::Relaxed) {
		let mut line = String::new();
		io::stdin().read_line(&mut line)?;

		let event : PlatoResponse = serde_json::from_str(&line)?;
		if !line.is_empty() {
			writeln!(logfile, "{}:\tStdinEvent: {event:?}", chrono::offset::Local::now());
		}
	}
	Ok(())
}
