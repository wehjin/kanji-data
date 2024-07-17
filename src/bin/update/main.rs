use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use records::{KanjiRecord, parse_kanji};

use crate::array_code::{array_array_code, array_code, Item, Kind};
use crate::build_string::BuildString;

pub mod build_string;
pub mod array_code;
pub mod records;

pub fn glyphs_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let kanji = kanji.as_ref();
	let values = kanji.iter()
		.map(|kanji| Item(kanji.kanji.to_string(), Kind::Str))
		.collect::<Vec<_>>();
	array_code("KA_GLYPHS", Kind::Str, values)
}
pub fn meanings_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let kanji = kanji.as_ref();
	let values = kanji.iter()
		.map(|kanji| Item(kanji.kmeaning.to_string(), Kind::Str))
		.collect::<Vec<_>>();
	array_code("KA_MEANINGS", Kind::Str, values)
}

pub fn onyomi_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let values = kanji.as_ref().iter()
		.map(|kanji| kanji.onyomi_ja.to_string())
		.collect::<Vec<_>>();
	array_array_code("KA_ONYOMIS", values)
}

pub fn kunyomi_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let values = kanji.as_ref().iter()
		.map(|kanji| kanji.kunyomi_ja.to_string())
		.collect::<Vec<_>>();
	array_array_code("KA_KUNYOMIS", values)
}
pub fn code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let glyphs = glyphs_array_code(&kanji);
	let meanings = meanings_array_code(&kanji);
	let onyomis = onyomi_array_code(&kanji);
	let kunyomis = kunyomi_array_code(&kanji);
	BuildString::new()
		.add_line(glyphs)
		.add_line(meanings)
		.add_line(onyomis)
		.add_line(kunyomis)
		.to_string()
}

pub fn main() -> anyhow::Result<()> {
	let out_path = out_dir()?.join("built.rs");
	let code = code(parse_kanji());
	File::create(out_path.clone())?.write_all(code.as_bytes())?;
	println!("Wrote to {}", out_path.to_str().unwrap());
	Ok(())
}

fn out_dir() -> anyhow::Result<PathBuf> {
	let current_dir = current_dir()?;
	let src_dir = current_dir.join("src");
	let write_dir = if src_dir.is_dir() { src_dir } else { current_dir };
	Ok(write_dir)
}

#[cfg(test)]
mod tests {
	mod shortbox {
		use crate::code;
		use crate::records::{KanjiRecord, parse_kanji};

		#[test]
		fn check_code() {
			let code = code(short_kanji());
			print!("{}", code);
		}

		fn short_kanji() -> Vec<KanjiRecord> {
			let mut kanji = parse_kanji();
			kanji.truncate(5);
			kanji
		}
	}
	mod sandbox {
		use crate::records::parse_kanji;

		use self::built::*;

		pub mod built {
			pub(crate) const KA_GLYPHS: [&'static str; 1] = [
				"示",
			];
			pub(crate) const KA_MEANINGS: [&'static str; 1] = [
				"show",
			];
			pub(crate) const KA_ONYOMIS: [&'static [&'static str]; 1] = [
				&["シ", "ジ"],
			];
		}
		pub struct SbKanjiData(pub usize);
		impl SbKanjiData {
			pub fn as_glyph(&self) -> &'static str { KA_GLYPHS[self.0] }
			pub fn as_meaning(&self) -> &'static str { KA_MEANINGS[self.0] }
			pub fn as_onyomi(&self) -> &'static [&'static str] { KA_ONYOMIS[self.0] }
		}
		#[test]
		fn check_kanji() {
			let all = parse_kanji();
			let record_at_4 = &all[4];
			dbg!(&record_at_4);

			let kanji = SbKanjiData(0);
			assert_eq!(kanji.as_glyph(), "示");
			assert_eq!(kanji.as_meaning(), "show");
			assert_eq!(kanji.as_onyomi().len(), 2);
			assert_eq!(kanji.as_onyomi()[0], "シ");
			assert_eq!(kanji.as_onyomi()[1], "ジ");
		}
	}
}
