use crate::built::{KA_GLYPHS, KA_MEANINGS};

pub mod records;
pub mod built;

pub struct Kanji(pub usize);
impl Kanji {
	pub fn as_glyph(&self) -> &'static str { KA_GLYPHS[self.0] }
	pub fn as_meaning(&self) -> &'static str { KA_MEANINGS[self.0] }
}

#[cfg(test)]
mod tests;
