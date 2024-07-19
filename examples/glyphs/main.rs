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
	/// Print the onyomi.
	#[clap(short, long, default_value_t = false)]
	onyo: bool,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	let count = args.count;
	if args.onyo {
		emit_onyo(count);
	} else {
		if count {
			emit_length()
		}
		emit_glyphs();
	}
	Ok(())
}

fn emit_onyo(count: bool) {
	let mut glyphs = (0..KanjiData::len())
		.into_iter()
		.map(KanjiData)
		.map(|data| {
			data.as_onyomi().iter().map(|&it| it.to_string()).collect::<Vec<_>>()
		})
		.flatten()
		.unique()
		.filter(|it| !it.is_empty())
		.collect::<Vec<_>>()
		;
	glyphs.sort();
	let glyphs_len = glyphs.len();
	let grouped = glyphs.into_iter().into_group_map_by(|it| it.chars().next().unwrap().to_string());
	let keys = grouped.keys().sorted().collect::<Vec<_>>();
	for key in keys.clone() {
		let group = grouped.get(key).unwrap();
		println!("(\"{}\", [\"{}\";{}]),", key, group.join("\",\""), group.len());
	}
	let keys = keys.into_iter().cloned().collect::<Vec<_>>();
	println!("[ \"{}\";{} ]", keys.join("\",\""), keys.len());
	if count {
		println!("{} onyomi", glyphs_len);
	}
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