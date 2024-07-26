use crate::built::*;
use crate::built_examples::{KA_EXAMPLES, KA_EXAMPLES_DATA};
use crate::examples::{ExamplesData, KanjiExample};

pub mod built;
pub mod built_examples;
pub mod examples;
pub mod records;

#[derive(Debug, Copy, Clone)]
pub struct KanjiData(pub usize);
impl KanjiData {
	pub fn as_glyph(&self) -> &'static str { KA_GLYPHS[self.0] }
	pub fn as_meaning(&self) -> &'static str { KA_MEANINGS[self.0] }
	pub fn as_onyomi(&self) -> &'static [&'static str] { KA_ONYOMIS[self.0] }
	pub fn as_kunyomi(&self) -> &'static [&'static str] { KA_KUNYOMIS[self.0] }
	pub fn as_examples(&self) -> &'static [KanjiExample<'static>] {
		let ExamplesData { first, count } = KA_EXAMPLES_DATA[self.0];
		&KA_EXAMPLES[first..(first + count)]
	}
	pub fn len() -> usize { KA_GLYPHS.len() }
}

#[cfg(test)]
mod tests;
