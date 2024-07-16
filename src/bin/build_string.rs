pub struct BuildString(Vec<String>);
impl BuildString {
	pub fn new() -> Self {
		Self(Vec::new())
	}
	pub fn add_line(mut self, line: impl AsRef<str>) -> Self {
		self.0.push(line.as_ref().to_string());
		self
	}
	pub fn add_line_with_take(self, f: impl Fn(Self) -> Self) -> Self {
		f(self)
	}
	pub fn to_string(self) -> String { self.0.join("\n") }
}
