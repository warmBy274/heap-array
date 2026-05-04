use std::{mem::replace, ops::{Deref, DerefMut, Index, IndexMut}};

#[derive(Clone)]
pub struct FnMap<K, V> {
    index_fn: fn(&K) -> usize,
    buckets: Vec<Option<(usize, (K, V))>>
}
impl<K, V> FnMap<K, V> {
    pub const fn new(index_fn: fn(&K) -> usize) -> Self {
        Self {
            index_fn,
            buckets: Vec::new()
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> () {
        if self.buckets.len() == 0 {
            self.buckets.push(Some(((self.index_fn)(&key), (key, value))));
            return;
        }
        let index = (self.index_fn)(&key);
        let bucket_index = index % self.buckets.len();
        if let Some((i, (_, v))) = &mut self.buckets[bucket_index] {
            if *i == index {
                *v = value;
            }
            else {
                self.resize();
                self.insert(key, value);
            }
        }
        else {
            self.buckets[bucket_index] = Some((index, (key, value)));
        }
    }
    pub fn contains(&self, key: &K) -> bool {
        if self.buckets.len() == 0 {return false;}
        let id = (self.index_fn)(key);
        let index = id % self.buckets.len();
        self.buckets[index].is_some()
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        if self.buckets.len() == 0 {return None;}
        let id = (self.index_fn)(key);
        let index = id % self.buckets.len();
        if let Some((pair_id, (_, v))) = &self.buckets[index] {
            if id == *pair_id {Some(v)}
            else {None}
        }
        else {None}
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.buckets.len() == 0 {return None;}
        let id = (self.index_fn)(key);
        let index = id % self.buckets.len();
        if let Some((pair_id, (_, v))) = &mut self.buckets[index] {
            if id == *pair_id {Some(v)}
            else {None}
        }
        else {None}
    }
    pub fn get_key_mut(&mut self, key: &K) -> Option<KeyMutGuard<'_, K, V>> {
        if self.buckets.len() == 0 {return None;}
        let id = (self.index_fn)(key);
        let index = id % self.buckets.len();
        if let Some((pair_id, (_, _))) = self.buckets[index] {
            if pair_id == id {
                Some(KeyMutGuard {
                    map: self,
                    index
                })
            }
            else {None}
        }
        else {None}
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.buckets.len() == 0 {return None;}
        let index = (self.index_fn)(key) % self.buckets.len();
        self.buckets[index].take().map(|(_, (_, v))| v)
    }
    pub fn iter_values(&self) -> impl Iterator<Item = &V> {
        self.buckets.iter().flatten().map(|(_, (_, v))| v)
    }
    pub fn iter_mut_values(&mut self) -> impl Iterator<Item = &mut V> {
        self.buckets.iter_mut().flatten().map(|(_, (_, v))| v)
    }
    pub fn iter_keys(&self) -> impl Iterator<Item = &K> {
        self.buckets.iter().flatten().map(|(_, (k, _))| k)
    }
    // here must be iter MUT keys
    pub fn iter_pairs(&self) -> impl Iterator<Item = &(K, V)> {
        self.buckets.iter().flatten().map(|(_, p)| p)
    }
    // here must be iter MUT pairs
    pub fn retain(&mut self, mut f: impl FnMut(&(K, V)) -> bool) -> () {
        for bucket in self.buckets.iter_mut() {
            if let Some((_, p)) = bucket {
                if !f(&*p) {*bucket = None;}
            }
        }
    }
    pub fn retain_mut(&mut self, mut f: impl FnMut(&mut (K, V)) -> bool) -> () {
        for bucket in self.buckets.iter_mut() {
            if let Some((_, p)) = bucket {
                if !f(p) {*bucket = None;}
            }
        }
    }
    pub fn clear(&mut self) -> () {
        self.buckets = Vec::new();
    }
    fn resize(&mut self) -> () {
        let new_len = self.buckets.len() * 2;
        let old_buckets = replace(&mut self.buckets, Vec::with_capacity(new_len));
        for _ in 0..new_len {
            self.buckets.push(None);
        }
        for element in old_buckets {
            if let Some((index, (key, value))) = element {
                self.buckets[index % new_len] = Some((index, (key, value)));
            }
        }
    }
}
impl<K, V: PartialEq> FnMap<K, V> {
    pub fn contains_value(&self, value: &V) -> bool {
        if self.buckets.len() == 0 {return false;}
        self.buckets.iter().any(|x| if let Some((_, (_, v))) = x {v == value} else {false})
    }
}
impl<K, V> Index<K> for FnMap<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        if let Some(v) = self.get(&index) {v}
        else {panic!("index not in FnMap");}
    }
}
impl<K, V> IndexMut<K> for FnMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        if let Some(v) = self.get_mut(&index) {v}
        else {panic!("index not in FnMap");}
    }
}

pub struct KeyMutGuard<'a, K, V> {
    map: &'a mut FnMap<K, V>,
    index: usize
}
impl<'a, K, V> Deref for KeyMutGuard<'a, K, V> {
    type Target = K;
    fn deref(&self) -> &Self::Target {
        if let Some((_, (k, _))) = &self.map.buckets[self.index] {k}
        else {unreachable!();}
    }
}
impl<'a, K, V> DerefMut for KeyMutGuard<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some((_, (k, _))) = &mut self.map.buckets[self.index] {k}
        else {unreachable!();}
    }
}
impl<'a, K, V> Drop for KeyMutGuard<'a, K, V> {
    fn drop(&mut self) {
        let pair = self.map.buckets[self.index].take().unwrap();
        self.map.insert(pair.1.0, pair.1.1);
    }
}