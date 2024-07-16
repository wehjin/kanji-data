use crate::built::*;

pub mod built;

#[derive(Debug, Copy, Clone)]
pub struct KanjiData(pub usize);
impl KanjiData {
	pub fn as_glyph(&self) -> &'static str { KA_GLYPHS[self.0] }
	pub fn as_meaning(&self) -> &'static str { KA_MEANINGS[self.0] }
	pub fn as_onyomi(&self) -> &'static [&'static str] { KA_ONYOMIS[self.0] }
	pub fn as_kunyomi(&self) -> &'static [&'static str] { KA_KUNYOMIS[self.0] }
	pub fn len() -> usize { KA_GLYPHS.len() }
}

#[cfg(test)]
mod tests;
