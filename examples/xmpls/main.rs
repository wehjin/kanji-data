use std::fs::File;
use std::io::Write;

use kanji_data::examples::{ExamplesData, KanjiExample};
use kanji_data::records::parse_kanji;

//noinspection SpellCheckingInspection
pub fn main() -> Result<(), anyhow::Error> {
	let records = parse_kanji();
	let mut pos_and_count = Vec::new();
	let mut example_parts = Vec::new();
	for record in records {
		let all_examples = record.examples.trim();
		let bracket_trimmed = &all_examples[4..all_examples.len() - 4];
		let first_pos = example_parts.len();
		for example_str in bracket_trimmed.split(" ], [ ").collect::<Vec<_>>() {
			let example_chars = example_str.chars().collect::<Vec<_>>();
			let example_chars = &example_chars[1..example_chars.len() - 1];
			let example_trimmed = example_chars.iter().collect::<String>();
			let parts = example_trimmed.split("\", \"").collect::<Vec<_>>();
			let meaning_part = parts[1].trim().to_string();
			let word_part = parts[0].trim().chars().collect::<Vec<_>>();
			let kanji_len = word_part.iter().position(|&it| it == '（').unwrap();
			let kanji_part = (&word_part[0..kanji_len]).iter().collect::<String>();
			let sound_start = kanji_len + 1;
			let sound_plus = &word_part[sound_start..];
			let sound_len = sound_plus.iter().position(|&it| it == '）').unwrap();
			let sound_part = &sound_plus[0..sound_len];
			let sound_part = sound_part.iter().collect::<String>();
			example_parts.push((kanji_part, sound_part, meaning_part));
		}
		let last_pos = example_parts.len();
		let count = last_pos - first_pos;
		pos_and_count.push((first_pos, count))
	}

	let mut examples = Vec::new();
	for (kanji, sound, meaning) in &example_parts {
		let example = KanjiExample {
			kanji: kanji.as_str(),
			sound: sound.as_ref(),
			meaning: meaning.as_str(),
		};
		examples.push(example);
	}

	let path = "target/debug/build/built_examples.rs";
	println!("Writing to {}", path);
	let mut file = File::create(path)?;
	writeln!(file, "// File generated by example 'xmpls'.")?;
	writeln!(file, "use crate::examples::{{ExamplesData, KanjiExample}};")?;
	writeln!(file, "// ALL EXAMPLES FROM ALL KANJI RECORDS")?;
	writeln!(file, "pub const KA_EXAMPLES: &[KanjiExample; {}] = &[", examples.len())?;
	for example in examples {
		writeln!(file, "    {:?},", example)?;
	}
	writeln!(file, "];")?;
	writeln!(file, "// EXAMPLES DATA PER KANJI RECORD")?;
	writeln!(file, "pub const KA_EXAMPLES_DATA: &[ExamplesData; {}] = &[", pos_and_count.len())?;
	for (pos, count) in pos_and_count {
		let data = ExamplesData { first: pos, count };
		writeln!(file, "    {:?},", data)?;
	}
	writeln!(file, "];")?;
	file.flush()?;
	Ok(())
}

