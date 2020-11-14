use std::collections::HashMap;
use std::cmp::{Eq, Ord};
use std::hash::Hash;

#[derive(Debug)]
pub struct Dictionary<K: Eq + Hash> {
  pool: HashMap<K, u16>,
  next_idx: u16
}

pub struct DictionaryIter<'a, K: 'a> {
  values: Vec<(&'a u16, &'a K)>,
  idx: usize
}

impl <'a, K> Iterator for DictionaryIter<'a, K> {
  type Item = (&'a u16, &'a K);

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx < self.values.len() {
      let i = self.idx;
      self.idx += 1;
      Some(self.values[i])
    } else {
      None
    }
  }
}

impl <K: Eq + Hash> Dictionary<K> {
  pub fn new() -> Dictionary<K> {
    Dictionary { 
      pool: HashMap::new(),
      next_idx: 1
    }
  }

  pub fn get(&self, idx: &u16) -> Option<&K> {
    for (element, i) in self.pool.iter() {
      if *i == *idx {
        return Some(element);
      }
    }
    None
  }

  pub fn map(&mut self, element: K) -> u16 {
    let mut new_idx = Some(self.next_idx);
    let entry = self.pool.entry(element).or_insert_with(|| {
      let idx = new_idx.unwrap();
      new_idx = None;
      idx
    });
    if new_idx.is_none() {
      self.next_idx += 1;
    }
    *entry
  }

  pub fn size(&self) -> u16 {
    self.next_idx
  }

  pub fn iter<'a>(&'a self) -> DictionaryIter<'a, K> {
    let mut elements: Vec<(&u16, &K)> = self.pool.iter()
      .map(|(element, idx)| (idx, element))
      .collect();
    elements.sort_by(|a, b| a.0.cmp(b.0));
    DictionaryIter { values: elements, idx: 0 }
  }
}
