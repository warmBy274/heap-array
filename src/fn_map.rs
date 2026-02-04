use std::mem::take;

pub struct FnMap<K, V> {
    index_fn: fn(&K) -> usize,
    buckets: Vec<Option<(usize, V)>>
}
impl<K, V: Clone> FnMap<K, V> {
    pub fn new(index_fn: fn(&K) -> usize) -> Self {
        Self {
            index_fn,
            buckets: Vec::new()
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> () {
        if self.buckets.len() == 0 {
            self.buckets.push(Some(((self.index_fn)(&key), value)));
            return;
        }
        let index = (self.index_fn)(&key);
        let bucket_index = index % self.buckets.len();
        if let Some((i, v)) = &mut self.buckets[bucket_index] {
            if *i == index {
                *v = value;
            }
            else {
                self.resize();
                self.insert(key, value);
            }
        }
        else {
            self.buckets[bucket_index] = Some((index, value));
        }
    }
    pub fn get(&self, key: K) -> Option<&V> {
        let index = (self.index_fn)(&key) % self.buckets.len();
        if let Some((_, v)) = &self.buckets[index] {Some(v)}
        else {None}
    }
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let index = (self.index_fn)(&key) % self.buckets.len();
        if let Some((_, v)) = &mut self.buckets[index] {Some(v)}
        else {None}
    }
    fn resize(&mut self) -> () {
        let old_buckets = take(&mut self.buckets);
        let new_len = old_buckets.len() * 2;
        self.buckets = vec![None; new_len];
        for element in old_buckets {
            if let Some((index, value)) = element {
                self.buckets[index % new_len] = Some((index, value));
            }
        }
    }
}