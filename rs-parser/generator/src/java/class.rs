use std::cmp::Eq;
use crate::java::constructs::{
  Attribute,
  Signature,
  Method
};
use crate::java::constants::{
  ArrayType,
  Type
};
use crate::java::dictionary::{Dictionary, DictionaryIter};

pub type PoolIdx = u16;

#[derive(Debug, PartialEq, Hash)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(PoolIdx),
  /// Integer constant
  /// Structure
  /// ```
  ///  1. Integer value
  /// ```
  Integer(u32),
  /// String constant
  /// Structure
  /// ```
  ///  1. Index to the UTF8 value
  /// ```
  String(PoolIdx),
  /// Info refering to a method
  /// Structure
  /// ```
  ///  1. Index to class info
  ///  2. INdex to a name & type info
  /// ```
  MethodRef(PoolIdx, PoolIdx),
  /// Info refering to a method of an interface
  /// Structure
  /// ```
  ///  1. Index to class info
  ///  2. INdex to a name & type info
  /// ```
  InterfaceMethodRef(PoolIdx, PoolIdx),
  /// Info about a function
  /// Structure
  /// ```
  ///  1. Index to the method name info
  ///  2. INdex to the descriptor info
  /// ```
  NameAndType(PoolIdx, PoolIdx)
}

impl Eq for PoolElement {}

type ClassPool = Dictionary<PoolElement>;

#[derive(Debug)]
pub struct JavaClass {
  class_pool: ClassPool,
  // TODO collect this information
  pub class_id: PoolIdx,
  pub super_class_id: PoolIdx,
  pub interfaces: Vec<PoolIdx>,
  pub methods: Vec<Method>
}

impl JavaClass {
  pub fn new() -> JavaClass {
    JavaClass {
      class_pool: ClassPool::new(),
      class_id: 0,
      super_class_id: 0,
      interfaces: Vec::new(),
      methods: Vec::new()
    }
  }

  pub fn set_class(&mut self, classname: &str) {
    self.class_id = self.map_class(classname);
  }

  pub fn set_super_class(&mut self, classname: &str) {
    self.super_class_id = self.map_class(classname);
  }

  pub fn get_class_name(&self) -> Option<String> {
    self.class_pool.get(&self.class_id)
      .and_then(|element| match element {
        &PoolElement::ClassInfo(ref idx) => Some(idx),
        _ => None
      })
      .and_then(|idx| self.class_pool.get(idx))
      .and_then(|element| match element {
        &PoolElement::Utf8Value(ref value) => Some(value.clone()),
        _ => None
      })
  }

  pub fn map_integer(&mut self, value: u32) -> PoolIdx {
    self.class_pool.map(PoolElement::Integer(value))
  }

  pub fn map_string(&mut self, value: &str) -> PoolIdx {
    let value_idx = self.map_utf8_value(value);
    self.class_pool.map(PoolElement::String(value_idx))
  }

  /// Creates a new method in the class
  ///
  /// Unlike #map_method that creates a reference to an existing
  /// method, this method creates a new method for the class.
  /// The declaration includes the operations executed by the method
  /// as well as its metadata
  pub fn create_method(
      &mut self,
      access: u16,
      method_name: &str,
      signature: Signature,
      attributes: Vec<Attribute>) -> PoolIdx {
    let name_idx = self.map_utf8_value(method_name);
    let descriptor = create_descriptor(&signature);
    let descriptor_idx = self.map_utf8_value(&descriptor);
    let attr_idx = self.map_utf8_value(&"Code");

    self.methods.push(Method {
      access: access,
      name_index: name_idx,
      descriptor_index: descriptor_idx,
      attributes: attributes.into_iter()
        .map(|attr| (attr_idx, attr))
        .collect()
    });

    self.map_self_method(method_name, &signature)
  }

  fn map_name_and_type(
      &mut self,
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let method_idx = self.map_utf8_value(method_name);
    let descr_idx = self.map_descriptor(signature);
    let name_and_type = PoolElement::NameAndType(method_idx, descr_idx);
    self.class_pool.map(name_and_type)
  }

  /// Maps a method existing in this or another concrete Class.
  ///
  /// It refers to the method by the class name, the method name
  /// and its signature
  pub fn map_method(
      &mut self,
      class_name: &str,
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let class_idx = self.map_class(class_name);
    let nnt_idx = self.map_name_and_type(method_name, signature);
    let method_ref = PoolElement::MethodRef(class_idx, nnt_idx);
    self.class_pool.map(method_ref)
  }

  /// Maps a method existing in this or another Interface.
  ///
  /// It refers to the interface method by the class name, the method name
  /// and its signature
  pub fn map_interface_method(
      &mut self,
      class_name: &str,
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let class_idx = self.map_class(class_name);
    let nnt_idx = self.map_name_and_type(method_name, signature);
    let method_ref = PoolElement::InterfaceMethodRef(class_idx, nnt_idx);
    self.class_pool.map(method_ref)
  }

  pub fn map_self_method(
      &mut self,
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let class_name = String::from(
      self.get_class_name().expect("Class name not defined yet"));
    self.map_method(
      &class_name,
      method_name,
      &signature)
  }

  pub fn map_class(&mut self, classname: &str) -> PoolIdx {
    let value_idx = self.map_utf8_value(classname);
    let info = PoolElement::ClassInfo(value_idx);
    self.class_pool.map(info)
  }

  fn map_descriptor(&mut self, signature: &Signature) -> PoolIdx {
    let descriptor = create_descriptor(signature);
    self.map_utf8_value(&descriptor)
  }

  pub fn map_utf8_value(&mut self, value: &str) -> PoolIdx {
    let info = PoolElement::Utf8Value(String::from(value));
    self.class_pool.map(info)
  }

  /// Gets an iterator on all elements of the class pool
  ///
  /// Elements are enumrated by increasing pool idx.
  pub fn pool_iter<'a>(&'a self) -> DictionaryIter<'a, PoolElement> {
    self.class_pool.iter()
  }

  /// Gets the size of the class pool
  pub fn pool_size(&self) -> PoolIdx {
    self.class_pool.size()
  }
}

fn create_descriptor(signature: &Signature) -> String {
  let mut descriptor = String::from("(");
  for param in &signature.parameter_types {
    type_to_str(&mut descriptor, param);
  }
  descriptor.push(')');
  { type_to_str(&mut descriptor, &signature.return_type); }

  descriptor
}

fn type_to_str(out: &mut String, t: &Type) {
  match t {
    &Type::Void => out.push('V'),
    &Type::Boolean => out.push('Z'),
    &Type::Integer => out.push('I'),
    &Type::Object(ref c) => {
      out.push('L');
      out.push_str(c);
      out.push(';');
    },
    &Type::ObjectArray(ref dim, ref object_type) => {
      (0..*dim).for_each(|_| out.push('['));
      out.push('L');
      out.push_str(object_type);
      out.push(';');
    },
    &Type::PrimitiveArray(ref dim, ref prim_type) => {
      (0..*dim).for_each(|_| out.push('['));
      match prim_type {
        &ArrayType::BOOLEAN => out.push('Z'),
        &ArrayType::CHAR => out.push('C'),
        &ArrayType::FLOAT => out.push('F'),
        &ArrayType::DOUBLE => out.push('D'),
        &ArrayType::BYTE => out.push('B'),
        &ArrayType::SHORT => out.push('S'),
        &ArrayType::INT => out.push('I'),
        &ArrayType::LONG => out.push('J')
      }
    }
  }
}

#[cfg(test)]
mod tests {

  mod pool {
    use super::super::*;

    #[test]
    fn test_map_new_value() {
      let mut pool = ClassPool::new();
      let i = pool.map(PoolElement::ClassInfo(1));
      let e = pool.get(&i);
      assert_eq!(e.unwrap(), &PoolElement::ClassInfo(1));
    }

    #[test]
    fn test_pool_size() {
      let mut pool = ClassPool::new();
      pool.map(PoolElement::ClassInfo(1));
      pool.map(PoolElement::ClassInfo(2));
      assert_eq!(pool.size(), 3);
    }

    #[test]
    fn test_map_multiple_values() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::ClassInfo(2));
      assert_ne!(i1, i2);
    }

    #[test]
    fn test_map_existing_value() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::ClassInfo(1));
      assert_eq!(i1, i2);
      assert_eq!(pool.size(), 2);
      assert_eq!(pool.get(&i1), Some(&PoolElement::ClassInfo(1)));
    }

    #[test]
    fn test_map_multiple_types() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::NameAndType(2, 3));
      assert_ne!(i1, i2);

      let e1 = pool.get(&i1).unwrap();
      assert_eq!(e1, &PoolElement::ClassInfo(1));
      let e2 = pool.get(&i2).unwrap();
      assert_eq!(e2, &PoolElement::NameAndType(2, 3));
    }

  }

  mod base {
    use super::super::*;
    use crate::java::constructs::Signature;
    use crate::java::constants::{Type, ArrayType, MethodAccess};

    #[test]
    fn test_set_class_name() {
      let mut c = JavaClass::new();
      c.set_class("a/b/C");
      assert_eq!(c.class_id, 2); // Mapping name then class info
    }

    #[test]
    fn test_get_class_name() {
      let mut c = JavaClass::new();
      c.set_class("a/b/C");
      assert_eq!(c.get_class_name().unwrap(), String::from("a/b/C"));
    }

    #[test]
    fn test_set_super_class_name() {
      let mut c = JavaClass::new();
      c.set_super_class("a/b/SC");
      assert_eq!(c.super_class_id, 2); // Mapping name then class info
    }

    #[test]
    fn test_class_and_super_names() {
      let mut c = JavaClass::new();
      c.set_class("a/b/C");
      c.set_super_class("a/b/SC");
      assert_eq!(c.class_id < c.super_class_id, true);
      assert_eq!(c.super_class_id < c.pool_size(), true);
      assert_eq!(c.pool_size(), 5);
    }

    #[test]
    fn test_map_integer() {
      let mut c = JavaClass::new();
      let idx = c.map_integer(132);
      assert_eq!(idx, 1u16);
      assert_eq!(c.pool_size(), 2);
      let integer = c.pool_iter().nth(0).expect("No item");
      assert_eq!(integer.0, &idx);
      assert_eq!(integer.1, &PoolElement::Integer(132));
    }

    #[test]
    fn test_map_many_integers() {
      let mut c = JavaClass::new();
      let idx_132 = c.map_integer(132);
      let idx_48 = c.map_integer(48);
      assert_ne!(idx_132, idx_48);
      let mut elements: Vec<Option<(&PoolIdx, &PoolElement)>> = vec![None; c.pool_size() as usize];
      {
        let elts = &mut elements;
        c.pool_iter().for_each(|e| {
          elts[*e.0 as usize] = Some(e);
        });
      }
      assert_eq!(
        elements[idx_132 as usize].unwrap(),
        (&idx_132, &PoolElement::Integer(132)));
      assert_eq!(
        elements[idx_48 as usize].unwrap(),
        (&idx_48, &PoolElement::Integer(48)));
    }

    #[test]
    fn test_map_method() {
      let mut c = JavaClass::new();
      let return_idx = c.map_method(
        &"a/C1",
        &"m1",
        &Signature {
          return_type: Type::ObjectArray(2, String::from("a/C2")),
          parameter_types: vec![
            Type::Integer,
            Type::PrimitiveArray(1, ArrayType::LONG)
          ]
        });

      assert_eq!(return_idx < c.pool_size(), true);
      assert_eq!(
        c.pool_size(),
        2 + // Class name and info
        2 + // method name and descriptor
        1 + // name_and_type
        1 + // method ref
        1); // as always for the count

      // Test the indexes
      let indexes: Vec<&u16> = c.pool_iter()
        .map(|elt| elt.0)
        .collect();
      for i in 0..(indexes.len() - 1) {
        let i1 = *indexes[i];
        let i2 = *indexes[i + 1];
        assert_eq!(
          i1 < i2,
          true,
          "Invalid indexes {} >= {}", i1, i2);
      }

      let elements: Vec<&PoolElement> = c.pool_iter()
        .map(|elt| elt.1)
        .collect();
      // Test the class info
      let class_info = elements.iter().find(|&elt| match elt {
        &&PoolElement::ClassInfo(_i) => true,
        _ => false
      }).expect(&format!("No class info in {:?}", elements));
      if let &PoolElement::ClassInfo(idx) = *class_info {
        assert_eq!(
          *elements[(idx - 1) as usize],
          PoolElement::Utf8Value(String::from("a/C1")));
      }

      // Test the name type
      let name_type = elements.iter().find(|&elt| match elt {
        &&PoolElement::NameAndType(_i, _j) => true,
        _ => false
      }).expect(&format!("No name & type in {:?}", elements));
      if let &PoolElement::NameAndType(m_idx, d_idx) = *name_type {
        assert_eq!(
          *elements[(m_idx - 1) as usize],
          PoolElement::Utf8Value(String::from("m1")));
        assert_eq!(
          *elements[(d_idx - 1) as usize],
          PoolElement::Utf8Value(String::from("(I[J)[[La/C2;")));
      }

      // Test the method ref
      let method_ref = elements.iter().find(|&elt| match elt {
        &&PoolElement::MethodRef(_i, _j) => true,
        _ => false
      }).expect(&format!("No method ref in {:?}", elements));
      if let &PoolElement::MethodRef(c_idx, nmt_idx) = *method_ref {
        assert_eq!(
          elements[(c_idx - 1) as usize],
          *class_info);
        assert_eq!(
          elements[(nmt_idx - 1) as usize],
          *name_type);
      }

      // Check that the return value match the method ref
      let ref_idx = c.pool_iter()
        .filter(|elt| elt.1 == *method_ref)
        .map(|elt| elt.0)
        .nth(0);
      assert_eq!(ref_idx, Some(&return_idx));
    }

    #[test]
    fn test_create_method() {
      let mut c = JavaClass::new();
      c.set_class("the/Class");
      let access: u16 =
        (MethodAccess::FINAL as u16) |
        (MethodAccess::PROTECTED as u16);
      let return_idx = c.create_method(
        access,
        "aMethod",
        Signature {
          return_type: Type::Integer,
          parameter_types: vec![]
        },
        vec![]);

      let pool_size = c.pool_size();
      {
        assert_eq!(return_idx < pool_size, true);
        assert_eq!(
          pool_size,
          2 + // Class name and info
          2 + // method name and descriptor
          1 + // name_and_type
          1 + // method ref
          1 + // Code keyword for method attribute
          1); // as always for the count

        let elements: Vec<&PoolElement> = c.pool_iter()
          .map(|elt| elt.1)
          .collect();
        // Test the class info
        let class_info = elements.iter().find(|&elt| match elt {
          &&PoolElement::ClassInfo(_i) => true,
          _ => false
        }).expect(&format!("No class info in {:?}", elements));
        if let &PoolElement::ClassInfo(idx) = *class_info {
          assert_eq!(
            *elements[(idx - 1) as usize],
            PoolElement::Utf8Value(String::from("the/Class")));
        }

        // Test the name type
        let name_type = elements.iter().find(|&elt| match elt {
          &&PoolElement::NameAndType(_i, _j) => true,
          _ => false
        }).expect(&format!("No name & type in {:?}", elements));
        if let &PoolElement::NameAndType(m_idx, d_idx) = *name_type {
          assert_eq!(
            *elements[(m_idx - 1) as usize],
            PoolElement::Utf8Value(String::from("aMethod")));
          assert_eq!(
            *elements[(d_idx - 1) as usize],
            PoolElement::Utf8Value(String::from("()I")));
        }

        // Test the method ref
        let method_ref = elements.iter().find(|&elt| match elt {
          &&PoolElement::MethodRef(_i, _j) => true,
          _ => false
        }).expect(&format!("No method ref in {:?}", elements));
        if let &PoolElement::MethodRef(c_idx, nmt_idx) = *method_ref {
          assert_eq!(
            elements[(c_idx - 1) as usize],
            *class_info);
          assert_eq!(
            elements[(nmt_idx - 1) as usize],
            *name_type);
        }

        // Check that the return value match the method ref
        let ref_idx = c.pool_iter()
          .filter(|elt| elt.1 == *method_ref)
          .map(|elt| elt.0)
          .nth(0);
        assert_eq!(ref_idx, Some(&return_idx));
      }

      {
        let code_idx = c.map_utf8_value("Code");
        assert_eq!(pool_size, c.pool_size());
        assert_eq!(code_idx < pool_size, true);
      }
    }
  }

}
