use crate::build_string::BuildString;

pub enum Kind {
	Str,
	ArrayStr,
}
impl Kind {
	pub fn to_code(&self) -> String {
		match self {
			Kind::Str => "str".to_string(),
			Kind::ArrayStr => "[&'static str]".to_string(),
		}
	}
}
pub struct Item(pub String, pub Kind);
impl Item {
	pub fn from_vec_strings(values: impl AsRef<[String]>) -> Self {
		let values = values.as_ref();
		let items = values.into_iter().map(|val| {
			Item(val.to_string(), Kind::Str).to_code()
		}).collect::<Vec<_>>();
		let val = items.join(",");
		Self(val, Kind::ArrayStr)
	}
	pub fn to_code(&self) -> String {
		match self.1 {
			Kind::Str => format!("\"{}\"", &self.0),
			Kind::ArrayStr => format!("&[{}]", &self.0),
		}
	}
}
pub fn array_code(name: impl AsRef<str>, kind: Kind, values: Vec<Item>) -> String {
	let name = name.as_ref();
	let build = BuildString::new()
		.add_line(format!("pub(crate) const {name}: [&'static {};{}] = [", kind.to_code(), values.len()))
		.add_line_with_take(|mut line_builder| {
			for val in &values {
				let line = format!("    {},", val.to_code());
				line_builder = line_builder.add_line(line);
			}
			line_builder
		})
		.add_line("];")
		.add_line("")
		.to_string()
		;
	build
}

pub fn array_array_code(name: impl AsRef<str>, values: Vec<String>) -> String {
	let values = values.iter()
		.map(|val| Item::from_vec_strings(val.split("、").map(String::from).collect::<Vec<_>>()))
		.collect::<Vec<_>>();
	array_code(name, Kind::ArrayStr, values)
}
