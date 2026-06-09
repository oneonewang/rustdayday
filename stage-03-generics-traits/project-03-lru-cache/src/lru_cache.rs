// 泛型 LRU 缓存实现
// 核心：HashMap<K, V> + Vec<K> 记录访问顺序
// get 触发"触碰"：把 key 从 order 移到末尾
// put 超容量时移除 order 头部（最久未用）

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub struct LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,         // 头部 = 最旧，尾部 = 最新
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LruCache capacity must be > 0, got {capacity}");
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// 获取 value。**会触碰 key**（把它移到最新位置）。
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get(key)
        } else {
            None
        }
    }

    /// 同 get，但返回可变引用。
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get_mut(key)
        } else {
            None
        }
    }

    /// 插入或覆盖。返回旧 value（如果有）。
    /// 超过容量时淘汰最久未用的。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let old = self.map.insert(key.clone(), value);
        if old.is_some() {
            self.touch(&key);
        } else {
            self.order.push(key);
            if self.map.len() > self.capacity {
                let oldest = self.order.remove(0);
                self.map.remove(&oldest);
            }
        }
        old
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let v = self.map.remove(key)?;
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        Some(v)
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// 按"从最旧到最新"顺序迭代 key
    pub fn iter_keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }

    /// 把 key 移到 order 末尾（最新位置）
    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let k = self.order.remove(pos);
            self.order.push(k);
        }
    }
}

impl<K, V> fmt::Debug for LruCache<K, V>
where
    K: Hash + Eq + Clone + fmt::Debug,
    V: Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LruCache")
            .field("len", &self.len())
            .field("capacity", &self.capacity)
            .field("order", &self.order)
            .field("map", &self.map)
            .finish()
    }
}
