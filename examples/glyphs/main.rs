use clap::Parser;
use itertools::Itertools;

use kanji_data::KanjiData;

/// This program prints all glyphs from the kanji data set.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Print the glyph count.
	#[clap(short, long, default_value_t = false)]
	count: bool,
	/// Print the kana and english.
	#[clap(short, long, default_value_t = false)]
	non_kanji: bool,
	/// Print the onyomi.
	#[clap(short, long, default_value_t = false)]
	onyo: bool,
	/// Print without quotes.
	#[clap(short, long, default_value_t = false)]
	plain: bool,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	let plain = args.plain;
	let count = args.count;
	let onyo = args.onyo;
	let non_kanji = args.non_kanji;
	if non_kanji {
		emit_kana(count);
	} else if onyo {
		emit_onyo(count, plain);
	} else {
		if count {
			emit_length()
		}
		emit_glyphs();
	}
	Ok(())
}

fn emit_kana(count: bool) {
	let strings = get_sorted_unique_strings(get_yomi_and_meaning_strings);
	let chars = strings.into_iter().collect::<String>().chars().into_iter()
		.unique()
		.sorted()
		.rev()
		.collect::<Vec<_>>();
	let len = chars.len();
	let string = chars.into_iter().collect::<String>();
	println!("{}", &string);
	if count {
		println!("{}", len);
	}
}

fn emit_onyo(count: bool, plain: bool) {
	let glyphs = get_sorted_unique_strings(get_onyomi_strings);
	let glyphs_len = glyphs.len();
	let grouped = glyphs.into_iter().into_group_map_by(|it| it.chars().next().unwrap().to_string());
	let keys = grouped.keys().sorted().collect::<Vec<_>>();
	for key in keys.clone() {
		let group = grouped.get(key).unwrap();
		println!("( \"{}\", [\"{}\"; {}] ),", key, group.join("\",\""), group.len());
	}
	let keys = keys.into_iter().cloned().collect::<Vec<_>>();
	if plain {
		println!("{}", keys.join(""));
	} else {
		println!("[ \"{}\"; {} ]", keys.join("\",\""), keys.len());
	}
	if count {
		println!("{} onyomi", glyphs_len);
	}
}

fn get_sorted_unique_strings(f: impl Fn(KanjiData) -> Vec<String>) -> Vec<String> {
	let mut glyphs = (0..KanjiData::len()).into_iter()
		.map(KanjiData)
		.map(f)
		.flatten()
		.unique()
		.filter(|it| !it.is_empty())
		.collect::<Vec<_>>()
		;
	glyphs.sort();
	glyphs
}

fn get_yomi_and_meaning_strings(data: KanjiData) -> Vec<String> {
	let meaning = data.as_meaning().to_string();
	let onyomi = get_onyomi_strings(data);
	let kunyomi = get_kunyomi_strings(data);
	let mut together = vec![meaning];
	together.extend(onyomi);
	together.extend(kunyomi);
	together
}

fn get_onyomi_strings(data: KanjiData) -> Vec<String> {
	data.as_onyomi().iter().map(|&it| it.to_string()).collect::<Vec<_>>()
}
fn get_kunyomi_strings(data: KanjiData) -> Vec<String> {
	data.as_kunyomi().iter().map(|&it| it.to_string()).collect::<Vec<_>>()
}

fn emit_length() {
	let count = KanjiData::len();
	println!("{}", count);
}

fn emit_glyphs() {
	let glyphs = (0..KanjiData::len())
		.into_iter()
		.map(KanjiData)
		.map(|data| data.as_glyph().chars().next().unwrap())
		.collect::<Vec<_>>()
		.into_iter().collect::<String>()
		;
	println!("{}", glyphs);
}