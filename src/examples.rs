#[derive(Debug, Copy, Clone)]
pub struct KanjiExample<'a> {
	pub kanji: &'a str,
	pub sound: &'a str,
	pub meaning: &'a str,
}

pub type ExamplePoint = usize;

#[derive(Debug, Copy, Clone)]
pub struct ExamplesData {
	pub first: ExamplePoint,
	pub count: usize,
}
