use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

use crate::error::{boxed, Result};

/// A config struct that holds all the configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub channels: Vec<(String, String)>,

	#[serde(skip, default = "String::new")]
	save_path: String,
}

impl Config {
	/// Attempts to create a new config file,
	/// reading from an existing file if found.
	pub fn new(path: &str) -> Result<Config> {
		Ok(
			match File::open(path) {
				Ok(f) => Some(f),
				Err(ref e) if e.kind() == ErrorKind::NotFound => None, // Not found files are ok
				Err(e) => return Err(Box::new(e)),
			}
			.map(BufReader::new) // Load to bufreader
			.map(serde_json::from_reader) // Deserialize
			.transpose()? // Transpose owo
			.map(|v| Config {
				save_path: path.to_owned(),
				..v
			}) // add the save_path
			.unwrap_or(Config {
				// Or else, return a new, empty config
				channels: Vec::new(),
				save_path: path.to_owned(),
			}),
		)
	}

	/// Saves the thing.
	pub fn save(&self) -> Result<()> {
		File::create(&self.save_path)
			.map(BufWriter::new)
			.map_err(boxed)
			.and_then(|f| serde_json::to_writer(f, self).map_err(boxed))
	}
}
