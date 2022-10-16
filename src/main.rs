use zotero::data_structure::item::Item;
use anyhow::{Error, Context, format_err};
use chrono;
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
use zotero::data_structure::shared_fields::Tagable;
use zotero::data_structure::collection::{Collection};
use std::ops::Deref;

mod plato_events;

use plato_events::PlatoEvent;

fn main() -> Result<(), Error> {

	let mut args = env::args().skip(1);

    //setup application logging
	let mut logfile: File;
	let logpath = Path::new("/var/log/zotero.log");
	if logpath.exists() {
		logfile = fs::OpenOptions::new().write(true).append(true).open(&logpath).expect("Error opening log file!");
	} else {
		logfile = File::create(&logpath).expect("Error creating log file!");
	}

    //register varibale to capture sigterm event
	let sigterm = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sigterm))?;

	writeln!(logfile, "{}:\t{args:?}", chrono::offset::Local::now());

	let zapi = ZoteroInit::set_user("8071408", "WhUroLO0NLtRY2BEkEVDP66t");

	//let mut filter_items = Vec::new();
	for item in zapi.get_items(None).expect("Error retrieving items") {
		if item.data.has_tag("plato-read".to_string()) {
			println!("{item:?}");
		}
	}
    println!("{}", PlatoEvent::message(&"Zotero Collections loaded!").to_json());

    //run until process is terminated by sigterm
	'mainloop: while !sigterm.load(Ordering::Relaxed) {
		let mut line = String::new();
		io::stdin().read_line(&mut line)?;

		let event : PlatoEvent = serde_json::from_str(&line)?;
		if !line.is_empty() {
			writeln!(logfile, "{}:\tStdinEvent: {event:?}", chrono::offset::Local::now());
		}
	}
	Ok(())
}
