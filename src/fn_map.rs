use std::{mem::replace, ops::{Index, IndexMut}};

pub struct FnMap<K, V> {
    index_fn: fn(&K) -> usize,
    buckets: Vec<Option<(usize, K, V)>>
}
impl<K, V> FnMap<K, V> {
    pub fn new(index_fn: fn(&K) -> usize) -> Self {
        Self {
            index_fn,
            buckets: Vec::new()
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> () {
        if self.buckets.len() == 0 {
            self.buckets.push(Some(((self.index_fn)(&key), key, value)));
            return;
        }
        let index = (self.index_fn)(&key);
        let bucket_index = index % self.buckets.len();
        if let Some((i, _, v)) = &mut self.buckets[bucket_index] {
            if *i == index {
                *v = value;
            }
            else {
                self.resize();
                self.insert(key, value);
            }
        }
        else {
            self.buckets[bucket_index] = Some((index, key, value));
        }
    }
    pub fn get(&self, id: usize) -> Option<&V> {
        let index = id % self.buckets.len();
        if let Some((_, _, v)) = &self.buckets[index] {Some(v)}
        else {None}
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut V> {
        let index = id % self.buckets.len();
        if let Some((_, _, v)) = self.buckets[index].as_mut() {
            Some(v)
        }
        else {None}
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            buckets: &self.buckets,
            pos: 0
        }
    }
    fn resize(&mut self) -> () {
        let new_len = self.buckets.len() * 2;
        let old_buckets = replace(&mut self.buckets, Vec::with_capacity(new_len));
        for _ in 0..new_len {
            self.buckets.push(None);
        }
        for element in old_buckets {
            if let Some((index, key, value)) = element {
                self.buckets[index % new_len] = Some((index, key, value));
            }
        }
    }
}
impl<K, V> Index<K> for FnMap<K, V> {
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        let index = (self.index_fn)(&key);
        if let Some(v) = self.get(index) {v}
        else {panic!("index {} not in FnMap", index);}
    }
}
impl<K, V> IndexMut<K> for FnMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        let index = (self.index_fn)(&key);
        if let Some(v) = self.get_mut(index) {v}
        else {panic!("index {} not in FnMap", index);}
    }
}
impl<K, V> IntoIterator for FnMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            buckets: self.buckets,
            pos: 0,
        }
    }
}

pub struct Iter<'a, K, V> {
    buckets: &'a [Option<(usize, K, V)>],
    pos: usize,
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.buckets.len() {
            match &self.buckets[self.pos] {
                Some((_, key, value)) => {
                    self.pos += 1;
                    return Some((key, value));
                }
                None => {
                    self.pos += 1;
                }
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.buckets.len() - self.pos;
        (0, Some(remaining))
    }
}

pub struct IntoIter<K, V> {
    buckets: Vec<Option<(usize, K, V)>>,
    pos: usize,
}
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.buckets.len() {
            let bucket = &mut self.buckets[self.pos];
            self.pos += 1;
            if let Some((_, key, value)) = bucket.take() {
                return Some((key, value));
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.buckets.len() - self.pos;
        (0, Some(remaining))
    }
}