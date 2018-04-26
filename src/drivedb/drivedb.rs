use super::Entry;
use regex::bytes::RegexSet;
use std::collections::HashSet;

#[derive(Debug)]
pub struct DriveDB<'a> {
	entries: Vec<&'a Entry>,

	// pre-found default entry: most likely it will be used right away, so it's not that harmful,
	// and it's better to have one if it's going to be requested multiple times
	default: Option<&'a Entry>,

	// precompiled RegexSets are often faster than simple regexes lazily compiled one by one on demand until the first match
	// (even if RegexSet compilation time is taken into account!),
	// and are a must if multiple lookups are about to be performed
	model_regexes: RegexSet,
	firmware_regexes: RegexSet,
}

impl<'a> DriveDB<'a> {
	pub fn new(entries: &'a Vec<Entry>) -> Self {
		let entries: Vec<_> = entries.iter()
			// USB ID entries are parsed differently; also, we don't support USB devices yet
			.filter(|e| ! e.model.starts_with("USB:"))
			.collect();

		let mut default = None;
		for entry in entries.iter() {
			if entry.family == "DEFAULT" {
				default = Some(*entry);
				break;
			}
		}

		// model and firmware are expected to be ascii strings, no need to try matching unicode characters
		let model_regexes = RegexSet::new(entries.iter()
			.map(|e| format!("^(?-u){}$", e.model))
		).unwrap();
		let firmware_regexes = RegexSet::new(entries.iter()
			.map(|e|
				if e.firmware.is_empty() {
					"(?-u)".to_string()
				} else {
					format!("^(?-u){}$", e.firmware)
				}
			)
		).unwrap();

		DriveDB {
			entries,
			default,
			model_regexes,
			firmware_regexes,
		}
	}
	pub fn find(&self, model: &str, firmware: &str) -> Option<&'a Entry> {
		let models: HashSet<_> = self.model_regexes.matches(model.as_bytes()).iter().collect();
		let firmwares: HashSet<_> = self.firmware_regexes.matches(firmware.as_bytes()).iter().collect();

		// find the first match (if any)
		models.intersection(&firmwares)
			.min()
			.map(|index| self.entries[*index])
	}
	/// Returns default entry from the database (if any).
	pub fn get_default_entry(&self) -> Option<&'a Entry> {
		self.default
	}
}
