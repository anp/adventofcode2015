use serde_json;
use serde_json::Value;

pub fn solve_part_one(input: &str) {
	let data: Value = serde_json::from_str(input).unwrap();
	
	let total = extract_total(data);
	println!("Santa's accounting software stored a total of {} in the document.", total);
}

fn extract_total(value: Value) -> i64 {
	match value {
		Value::Null => 0,
		Value::Bool(_) => 0,
		Value::I64(i) => i,
		Value::U64(u) => u as i64,
		Value::F64(_) => 0,
		Value::String(_) => 0,
		Value::Array(a) => {
			let mut sub_total = 0;
			for v in a {
				sub_total += extract_total(v);
			}
			sub_total
		},
		Value::Object(o) => {
			let mut sub_total = 0;
			for (_, v) in o {
				sub_total += extract_total(v);
			}
			sub_total
		},
	}
}