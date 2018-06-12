use std::collections::HashMap;

lazy_static! {
	pub static ref ARRAY_TYPES: HashMap<u8, &'static str> = {
    let mut m = HashMap::new();
    m.insert(4, "BOOLEAN");
    m.insert(5, "CHAR");
    m.insert(6, "FLOAT");
    m.insert(7, "DOUBLE");
    m.insert(8, "BYTE");
    m.insert(9, "SHORT");
    m.insert(10, "INT");
    m.insert(11, "LONG");
    m
  };
}