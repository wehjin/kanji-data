use std::cmp::Ordering;

use serde::Deserialize;

pub fn parse_kanji() -> Vec<KanjiRecord> {
	let mut records = vec![];
	let string = include_str!("../kanji-data-media/language-data/ka_data.csv");
	let mut reader = csv::Reader::from_reader(string.as_bytes());
	for result in reader.deserialize() {
		let record: KanjiRecord = result.expect("record");
		records.push(record);
	}
	records.sort_by(|a, b| {
		match a.kstroke.cmp(&b.kstroke) {
			Ordering::Less => Ordering::Less,
			Ordering::Equal => {
				let grade = (a.kgrade.parse::<usize>(), b.kgrade.parse::<usize>());
				let cmp_grade = match grade {
					(Ok(a), Ok(b)) => a.cmp(&b),
					(Ok(_), Err(_)) => Ordering::Less,
					(Err(_), Ok(_)) => Ordering::Greater,
					(Err(_), Err(_)) => Ordering::Equal
				};
				match cmp_grade {
					Ordering::Less => Ordering::Less,
					Ordering::Equal => {
						match a.kmeaning.len().cmp(&b.kmeaning.len()) {
							Ordering::Less => Ordering::Less,
							Ordering::Equal => a.kanji.cmp(&b.kanji),
							Ordering::Greater => Ordering::Greater,
						}
					}
					Ordering::Greater => Ordering::Greater,
				}
			}
			Ordering::Greater => Ordering::Greater,
		}
	});
	records
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct KanjiRecord {
	pub kanji: String,
	pub kname: String,
	pub kstroke: u8,
	pub kmeaning: String,
	pub kgrade: String,
	pub kunyomi_ja: String,
	pub kunyomi: String,
	pub onyomi_ja: String,
	pub onyomi: String,
	pub examples: String,
	pub radical: String,
	pub rad_order: f32,
	pub rad_stroke: String,
	pub rad_name_ja: String,
	pub rad_name: String,
	pub rad_meaning: String,
	pub rad_position_ja: String,
	pub rad_position: String,
}

impl KanjiRecord {
	pub fn to_onyomi_ja_vec(&self) -> Vec<String> {
		self.onyomi_ja
			.split("、")
			.map(String::from)
			.collect::<Vec<_>>()
	}
	pub fn to_kunyomi_ja_vec(&self) -> Vec<String> {
		self.kunyomi_ja
			.split("、")
			.map(String::from)
			.collect()
	}
}