lazy_static! {
	static ref CLASS_ACCESS_FLAGS: Vec<(&'static str, u16)> = {
		vec![
		("PUBLIC", 0x0001), // Declared public; may be accessed from outside its package.
		("FINAL", 0x0010), // Declared final; no subclasses allowed.
		("SUPER", 0x0020), // Treat superclass methods specially when invoked by the invokespecial instruction.
		("INTERFACE", 0x0200), // Is an interface, not a class.
		("ABSTRACT", 0x0400), // Declared abstract; must not be instantiated.
		("SYNTHETIC", 0x1000), // Declared synthetic; not present in the source code.
		("ANNOTATION", 0x2000), // Declared as an annotation type.
		("ENUM", 0x4000), // Declared as an enum type.
		("MODULE", 0x8000)  // Is a module, not a class or interface.
		]
	};
}

// pub fn to_class_access(flag_value: u16) -> Vec<&'static str> {
// 	CLASS_ACCESS_FLAGS.iter()
// 		.filter(|entry| flag_value & entry.1 != 0)
// 		.map(|&(name, _)| name)
// 		.collect()
// }


lazy_static! {
	static ref METHOD_ACCESS_FLAGS: Vec<(&'static str, u16)> = {
		vec![
  		("PUBLIC", 0x0001), // Declared public; may be accessed from outside its package.
  		("PRIVATE", 0x0002), // Declared private; accessible only within the defining class.
  		("PROTECTED", 0x0004), // Declared protected; may be accessed within subclasses.
  		("STATIC", 0x0008), // Declared static.
  		("FINAL", 0x0010), // Declared final; must not be overridden (§5.4.5).
  		("SYNCHRONIZED", 0x0020), // Declared synchronized; invocation is wrapped by a monitor use.
  		("BRIDGE", 0x0040), // A bridge method, generated by the compiler.
  		("VARARGS", 0x0080), // Declared with variable number of arguments.
  		("NATIVE", 0x0100), // Declared native; implemented in a language other than the Java programming language.
  		("ABSTRACT", 0x0400), // Declared abstract; no implementation is provided.
  		("STRICT", 0x0800), // Declared strictfp; floating-point mode is FP-strict.
  		("SYNTHETIC", 0x1000) // Declared synthetic; not present in the source code.
		]
	};
}

macro_rules! to_names {
	($name: ident, $values: tt) => {
		pub fn $name(flag_value: u16) -> Vec<&'static str> {
			$values.iter()
				.filter(|entry| flag_value & entry.1 != 0)
				.map(|&(name, _)| name)
				.collect()
		}
	};
}
to_names!(to_class_access, CLASS_ACCESS_FLAGS);
to_names!(to_method_access, METHOD_ACCESS_FLAGS);

// pub fn to_method_access(flag_value: u16) -> Vec<&'static str> {
// 	CLASS_ACCESS_FLAGS.iter()
// 		.filter(|entry| flag_value & entry.1 != 0)
// 		.map(|&(name, _)| name)
// 		.collect()
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_class_access_for_empty() {
		let flags = to_class_access(0x0u16);
		let empty: Vec<&'static str> = vec![];
		assert_eq!(flags, empty);
	}

	#[test]
	fn test_to_class_access() {
		let flags = to_class_access(0x421u16);
		assert_eq!(flags, vec!["PUBLIC", "SUPER", "ABSTRACT"]);
	}

	#[test]
	fn test_to_method_access_for_empty() {
		let flags = to_method_access(0x0u16);
		let empty: Vec<&'static str> = vec![];
		assert_eq!(flags, empty);
	}

	#[test]
	fn test_to_method_access() {
		let flags = to_method_access(0x132u16);
		assert_eq!(flags, vec!["PRIVATE", "FINAL", "SYNCHRONIZED", "NATIVE"]);
	}
}