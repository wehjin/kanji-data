use std::fs::File;
use std::io::{Read, Write};

use kanji_data::records::{KanjiRecord, parse_kanji};

use crate::build_string::BuildString;

pub mod build_string;

fn array_code(name: impl AsRef<str>, values: Vec<String>) -> String {
	let name = name.as_ref();
	let build = BuildString::new()
		.add_line(format!(
			"pub(crate) const {}: [&'static str;{}] = [",
			name,
			values.len()
		))
		.add_line_with_take(|mut line_builder| {
			for val in &values {
				let line = format!("    \"{}\",", val);
				line_builder = line_builder.add_line(line);
			}
			line_builder
		})
		.add_line("];")
		.add_line("")
		.to_string()
		;
	build
}

pub fn glyphs_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let kanji = kanji.as_ref();
	let glyphs = kanji.iter().map(|kanji| kanji.kanji.to_string()).collect::<Vec<_>>();
	let code = array_code("KA_GLYPHS", glyphs);
	code
}
pub fn code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let glyphs = glyphs_array_code(&kanji);
	let meanings = meanings_array_code(&kanji);
	BuildString::new()
		.add_line(glyphs)
		.add_line(meanings)
		.to_string()
}

pub fn meanings_array_code(kanji: impl AsRef<[KanjiRecord]>) -> String {
	let kanji = kanji.as_ref();
	let values = kanji.iter().map(|kanji| kanji.kmeaning.to_string()).collect::<Vec<_>>();
	let code = array_code("KA_MEANINGS", values);
	code
}

pub fn main() -> anyhow::Result<()> {
	let path = "src/built.rs";
	let mut kanji = parse_kanji();
	kanji.truncate(5);
	let code = code(kanji);
	File::create(path)?.write_all(code.as_bytes())?;
	{
		let mut file = File::open(path)?;
		let mut buf = String::new();
		file.read_to_string(&mut buf)?;
		print!("{}", &buf);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	mod live {
		use kanji_data::records::{KanjiRecord, parse_kanji};

		use crate::code;

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
		use kanji_data::records::parse_kanji;

		use crate::tests::sandbox::built::KA_MEANINGS;

		use self::built::KA_GLYPHS;

		pub mod built {
			pub(crate) const KA_GLYPHS: [&'static str; 1] = [
				"示",
			];
			pub(crate) const KA_MEANINGS: [&'static str; 1] = [
				"show",
			];
		}
		pub struct SbKanji(pub usize);
		impl SbKanji {
			pub fn as_glyph(&self) -> &'static str { KA_GLYPHS[self.0] }
			pub fn as_meaning(&self) -> &'static str { KA_MEANINGS[self.0] }
		}
		#[test]
		fn check_kanji() {
			let all = parse_kanji();
			let record_at_4 = &all[4];
			dbg!(&record_at_4);

			let kanji = SbKanji(0);
			assert_eq!(kanji.as_glyph(), "示");
			assert_eq!(kanji.as_meaning(), "show");
		}
	}
}
