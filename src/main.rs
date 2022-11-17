use anyhow::{Error, Context, format_err};
use chrono;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};
use std::io;
use std::io::{Write, Read, Cursor};
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use zotero::{ZoteroInit, Get};
use zotero::data_structure::ToJson;
use zotero::data_structure::collection::{Collection};
use zotero::data_structure::item::{Item, ItemType};
use zotero::data_structure::item::AttachmentData;
use std::ops::Deref;
use rustydav::client::Client;
use rustydav::prelude::*;
use toml;
use zip::ZipArchive;

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

fn download_pdf(parent_key: &String, pdf_id: &Vec<AttachmentData>, client: &Client, url: &str) -> Option<String> {
	for atch in pdf_id {
		println!("{}", format!("{}/{}.zip", url, atch.key));
		match client.get(&format!("{}/{}.zip", url, atch.key)) {
			Ok(resp) => {
				let mut archive = ZipArchive::new(Cursor::new(resp.bytes().unwrap())).unwrap();
				for i in 0..archive.len() {
					println!("{}",archive.by_index(i).unwrap().name());
				}
			}
			Err(err) => {}
		}
	}
	None
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

	let save_path = Path::new(args.get(1).unwrap());
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
	let mut pdf_attachments_children = BTreeMap::new();
	for i in &items_to_read {
		if i.meta.has_children() {
			let children: Vec<AttachmentData> = zapi.get_child_items(i.key(), None).unwrap().into_iter().filter_map(|item| {
				match item.data {
					ItemType::Attachment(atch) => {
						if &atch.content_type == "application/pdf" {
							Some(atch)
						} else {
							None
						}
					}
					_ => None
				}
			}).collect();
			pdf_attachments_children.insert(i.key().to_string(), children);
		}
	}
    println!("{}", PlatoMessage::notify(&"Zotero Items tagged for Reading loaded!").to_json());

	let webdav_client = Client::init(&settings.webdav_user, &settings.webdav_password);

	println!("{}", PlatoMessage::serach(&save_path.to_str().unwrap(), &"", &"", false).to_json());
    //run until process is terminated by sigterm
	'mainloop: while !sigterm.load(Ordering::Relaxed) {
		let mut line = String::new();
		io::stdin().read_line(&mut line)?;

		let mut plato_resp: Option<PlatoResponse> = None;
		if !line.is_empty() {
			match serde_json::from_str::<PlatoResponse>(&line) {
				Err(msg) => {
					writeln!(logfile, "{}:\tError: {msg}", chrono::offset::Local::now());
				}
				Ok(response) => {
					writeln!(logfile, "{}:\t{response:#?}", chrono::offset::Local::now());
					plato_resp = Some(response);
				}
			}
		}

		for (parent_key, pdf_attachments) in &pdf_attachments_children {
			download_pdf(parent_key, pdf_attachments, &webdav_client, &settings.webdav_url);
		}

	}
	Ok(())
}
