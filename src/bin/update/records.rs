use serde::Deserialize;

pub fn parse_kanji() -> Vec<KanjiRecord> {
	let mut records = vec![];
	let string = include_str!("../../../kanji-data-media/language-data/ka_data.csv");
	let mut reader = csv::Reader::from_reader(string.as_bytes());
	for result in reader.deserialize() {
		let record: KanjiRecord = result.expect("record");
		records.push(record);
	}
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