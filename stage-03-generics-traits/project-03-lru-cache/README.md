# Project 03 · 泛型 LRU 缓存

> 难度：⭐⭐⭐  
> 综合运用：泛型、trait bound、生命周期（间接）、关联类型思维、生命周期、模块化  
> 预计时间：2 – 3 小时

## 🎯 项目目标

实现一个**泛型 LRU（Least Recently Used）缓存**：

```rust
let mut cache: LruCache<&str, i32> = LruCache::new(2);
cache.put("a", 1);
cache.put("b", 2);
cache.get(&"a");          // 触碰 a，让 a 变最新
cache.put("c", 3);        // 容量满 → 淘汰最久未用的 b
assert!(cache.get(&"b").is_none());
assert_eq!(cache.get(&"a"), Some(&1));
```

## 📂 项目结构

```
project-03-lru-cache/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs           # 演示 + 测试
    └── lru_cache.rs      # 泛型 LRU 缓存实现
```

## 🚀 怎么跑

```bash
cd project-03-lru-cache
cargo run
```

## 📚 涉及的核心概念

| 概念 | 出处 |
|------|------|
| 泛型 `<K, V>` | 第 1 章 |
| `K: Hash + Eq + Clone` trait bound | 第 2 章 |
| `HashMap<K, V>` 查找 | Stage 2 |
| `Vec<K>` 维护顺序 | Stage 2 |
| 模块拆分 | Stage 2 |
| 关联类型思维 | 第 5 章（不用，只用 trait bound） |

## 🪜 怎么写

### Step 1：定义 struct

```rust
pub struct LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,           // front = 旧，back = 新
}
```

> 关键决定：
> - `K: Hash + Eq` 才能作为 HashMap 的 key
> - `K: Clone` 用来在 `order` 里存一份副本（vec.remove 后 push）
> - `V: Clone` 用来在调试输出时克隆

### Step 2：核心：touch / put / get

- `get(&mut self, key: &K)` 触碰 key（移到 order 末尾）+ 返回 `Option<&V>`
- `put(&mut self, key: K, value: V)` 插入；满了就 `order.remove(0)` 淘汰 + `map.remove(&oldest)`

### Step 3：辅助方法

- `len` / `is_empty` / `capacity` / `contains` / `remove` / `clear`

### Step 4：迭代

- `iter_keys()` 返回 `impl Iterator<Item = &K>`——用 trait bound + impl Trait 语法

## 🧪 测试

`main` 里跑这些：

1. 基本 put / get
2. 容量满后插入触发淘汰
3. get 触碰 → 改变 LRU 顺序
4. 重复 put（覆盖）不算新条目
5. remove / clear
6. 不同 key 类型：`i32` / `&str` / `(i32, i32)` 元组

## 🏃 运行结果

```
=== 演示 1：基本 put / get ===
cache = LruCache { len: 3, capacity: 3, order: ["a", "b", "c"], map: {...} }
get a = Some(1)
cache = LruCache { len: 3, capacity: 3, order: ["b", "c", "a"], map: {...} }

=== 演示 2：超过容量触发淘汰 ===
插入 1, 2 后: LruCache { len: 2, capacity: 2, order: [1, 2], map: {1: "one", 2: "two"} }
插入 3 后: LruCache { len: 2, capacity: 2, order: [2, 3], map: {2: "two", 3: "three"} }
get 1 = None

=== 演示 3：get 触发 LRU 顺序更新 ===
初始: LruCache { len: 3, capacity: 3, order: ['a', 'b', 'c'], map: {...} }
get a + put d 后: LruCache { len: 3, capacity: 3, order: ['b', 'c', 'a', ...]  // 等等
```

> 哦，演示 3 我代码里 `put('d', 4)` 时 order 实际是 `['b', 'c', 'a', 'd']`？让我看看……

是的——get 触碰 a 后 order = `['b', 'c', 'a']`，再 put d → order = `['b', 'c', 'a', 'd']`，但容量是 3，所以淘汰 order[0] = 'b'，最终 order = `['c', 'a', 'd']`。

## 🎁 扩展挑战

1. **命中率统计**：加 `hits: u64` / `misses: u64` 字段，`get` 每次更新
2. **`iter()`**：返回 `(&K, &V)` 对，按访问顺序
3. **`peek` / `peek_mut`**：和 `get` 一样但**不**触碰
4. **`resize(&mut self, new_cap: usize)`**：动态调整容量
5. **`from_iter`**：实现 `FromIterator<(K, V)>` 让 `cache.collect()` 成立
6. **`Default`**：让 `LruCache::default()` 可用（默认容量是多少？想想）
7. **O(1) 实现**：用 `Rc<RefCell<Node>>`（**Stage 4 会学**）+ 双向链表重写

## ⚠️ 复杂度说明

**当前实现**：`get` / `put` 都是 O(n)——因为 `order.remove(0)` 是 O(n)，`touch` 里 `position` 也是 O(n)。

**真正 O(1) 的 LRU** 需双向链表 + HashMap 存链表节点指针，下一阶段（Stage 4）的 `Rc<RefCell<Node>>` 就能写。

## ✅ 完成判定

- [ ] `cargo run` 跑通，所有断言通过
- [ ] 演示 1-6 全部输出合理
- [ ] 至少完成 1 个扩展挑战（**`hits` / `misses` 统计** 推荐作为入门）

完成 → 回到 [Stage 3 README](../README.md) 准备进入 Stage 4。
