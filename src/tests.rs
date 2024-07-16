use crate::Kanji;

#[test]
fn check_kanji() {
	let kanji = Kanji(4);
	assert_eq!(kanji.as_glyph(), "示");
	assert_eq!(kanji.as_meaning(), "show");
}
