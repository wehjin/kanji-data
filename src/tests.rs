use crate::KanjiData;

#[test]
fn check_kanji() {
	let kanji = KanjiData(4);
	assert_eq!(kanji.as_glyph(), "示");
	assert_eq!(kanji.as_meaning(), "show");
	assert_eq!(kanji.as_onyomi().len(), 2);
	assert_eq!(kanji.as_onyomi()[0], "シ");
	assert_eq!(kanji.as_onyomi()[1], "ジ");
	assert_eq!(kanji.as_kunyomi().len(), 2);
	assert_eq!(kanji.as_kunyomi()[0], "しめ");
	assert_eq!(kanji.as_kunyomi()[1], "しめす");

	let kanji = KanjiData(0);
	assert_eq!(kanji.as_glyph(), "一");
	assert_eq!(kanji.as_meaning(), "one");
	assert_eq!(kanji.as_onyomi().len(), 1);
	assert_eq!(kanji.as_onyomi()[0], "イチ");
	assert_eq!(kanji.as_kunyomi().len(), 1);
	assert_eq!(kanji.as_kunyomi()[0], "ひと");

	let examples = kanji.as_examples();
	assert_eq!(examples.len(), 9);
	assert_eq!(examples[0].kanji, "一年生");
	assert_eq!(examples[0].sound, "いちねんせい");
	assert_eq!(examples[0].meaning, "first-year student");
}

#[test]
fn check_len() {
	assert_eq!(KanjiData::len(), 1235);
}
