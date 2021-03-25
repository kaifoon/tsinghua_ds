//! A hash map implemented with two-way quadratic probing with random algorithm
//! HashMap len is fixed size by created

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash, Hasher};
use core::mem::{replace, take};
use core::ops::Index;

#[derive(Default, Debug)]
pub struct RandomState;

#[derive(Default, Debug)]
pub struct MyHasher {
    buffer: u64,
}

impl Hasher for MyHasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut buffer_array: [u8; 8] = [97, 107, 131, 211, 227, 241, 179, 151];
        let mut i = 0;
        for byte in bytes {
            buffer_array[i] ^= byte;
            i = i + 1 % 8;
        }

        self.buffer = unsafe {
            let mut result: u64 = core::mem::zeroed();
            core::ptr::copy_nonoverlapping(
                &buffer_array as *const [u8; 8] as *const u8,
                &mut result as *mut u64 as *mut u8,
                core::mem::size_of::<u64>(),
            );
            result
        };
    }

    fn finish(&self) -> u64 {
        self.buffer
    }
}

impl BuildHasher for RandomState {
    type Hasher = MyHasher;
    fn build_hasher(&self) -> MyHasher {
        MyHasher::default()
    }
}

fn make_hash<K: Hash + ?Sized>(hash_builder: &impl BuildHasher, val: &K) -> u64 {
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

/// HashMap
#[derive(Debug)]
pub struct HashMap<K, V, S = RandomState> {
    table: Vec<Option<(K, V)>>,
    hash_builder: S,
}

impl<K: Eq + Hash + Clone, V: Clone + Default> HashMap<K, V> {
    /// Create an empty `HashMap`
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tsinghua_ds::HashMap;
    ///
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    /// ```
    pub fn new() -> Self {
        // 4k + 3 , such as 2048 + 3 = 2051
        let table = vec![None; 2051];
        Self {
            table,
            hash_builder: RandomState::default(),
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical. See the [module-level
    /// documentation] for more.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use tsinghua_ds::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash(&self.hash_builder, &k);
        let offset = (hash % self.len() as u64) as usize;

        if self.table.get(offset).unwrap().is_none() {
            self.table[offset] = Some((k, v));
            return None;
        }

        // two ways square probing
        for i in 0..=self.table.len() / 2 {
            let idx = ((offset as i64 + (i * i) as i64) % self.len() as i64) as usize;
            if let Some(elem_ref) = self.table.get_mut(idx) {
                if let Some((key, item)) = elem_ref.as_mut() {
                    if k.eq(key) {
                        return Some(replace(item, v));
                    }
                } else {
                    self.table[idx] = Some((k, v));
                    return None;
                }
            }

            let idx = ((((offset as i64 - (i * i) as i64) % self.len() as i64) + self.len() as i64)
                % self.len() as i64) as usize;
            if let Some(elem_ref) = self.table.get_mut(idx) {
                if let Some((key, item)) = elem_ref.as_mut() {
                    if k.eq(key) {
                        return Some(replace(item, v));
                    }
                } else {
                    self.table[idx] = Some((k, v));
                    return None;
                }
            }
        }

        None
    }

    /// Creates an empty `HashMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use tsinghua_ds::HashMap;
    /// let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity / 4 + 3;
        let table = vec![None; capacity];

        Self {
            table,
            hash_builder: RandomState::default(),
        }
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Returns the number of elements in the map
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tsinghua_ds::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let hash = make_hash(&self.hash_builder, k);
        let offset = (hash % self.len() as u64) as usize;

        // two ways square probing
        for i in 0..=self.table.len() / 2 {
            let idx = ((offset as i64 + (i * i) as i64) % self.len() as i64) as usize;
            if let Some(Some(elem_ref)) = self.table.get_mut(idx) {
                if k.eq(elem_ref.0.borrow()) {
                    let result = take(&mut elem_ref.1);
                    self.table[idx].take();
                    return Some(result);
                }
            }

            let idx = ((((offset as i64 - (i * i) as i64) % self.len() as i64) + self.len() as i64)
                % self.len() as i64) as usize;
            if let Some(Some(elem_ref)) = self.table.get_mut(idx) {
                if k.eq(elem_ref.0.borrow()) {
                    let result = take(&mut elem_ref.1);
                    return Some(result);
                }
            }
        }
        None
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tsinghua_ds::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let hash = make_hash(&self.hash_builder, k);
        let offset = (hash % self.len() as u64) as usize;

        // two ways square probing
        for i in 0..=self.table.len() / 2 {
            let idx = ((offset as i64 + (i * i) as i64) % self.len() as i64) as usize;
            if let Some(Some(elem_ref)) = self.table.get(idx) {
                if k.eq(elem_ref.0.borrow()) {
                    return Some(&elem_ref.1);
                }
            }

            let idx = ((((offset as i64 - (i * i) as i64) % self.len() as i64) + self.len() as i64)
                % self.len() as i64) as usize;
            if let Some(Some(elem_ref)) = self.table.get(idx) {
                if k.eq(elem_ref.0.borrow()) {
                    return Some(&elem_ref.1);
                }
            }
        }
        None
    }
}

impl<K, Q: ?Sized, V: Clone + Default> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q> + Clone,
    Q: Eq + Hash,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_insert() {
        let mut map: HashMap<i32, String> = HashMap::new();

        for i in 0..2000 {
            let idx = 23338812 + i * i + 2 * i;
            let b = format!("{}", i);
            map.insert(idx, b.clone());
            assert_eq!(map.get(&idx), Some(&b));
        }

        for i in 0..2000 {
            let idx = 23338812 + i * i + 2 * i;
            let b = format!("{}", i);
            assert_eq!(map.get(&idx), Some(&b));
        }
    }
    #[test]
    fn hash_remove() {
        let mut map: HashMap<i32, String> = HashMap::new();

        for i in 0..2000 {
            let idx = 23338812 + i * i + 2 * i;
            let b = format!("{}", i);
            map.insert(idx, b);
        }

        for i in 0..2000 {
            let idx = 23338812 + i * i + 2 * i;
            let b = format!("{}", i);
            assert_eq!(map.remove(&idx), Some(b));
        }
    }
}
