use clap::Parser;

use kanji_data::KanjiData;

/// This program prints all glyphs from the kanji data set.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Print the glyphs count.
	#[clap(short, long, default_value_t = false)]
	count: bool,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	if args.count {
		emit_length()
	}
	emit_glyphs();
	Ok(())
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