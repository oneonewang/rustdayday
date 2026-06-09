// Project 03 · 泛型 LRU 缓存
// 任务：见 README.md

mod lru_cache;

use lru_cache::LruCache;

fn main() {
    // 演示 1：基本用法
    println!("=== 演示 1：基本 put / get ===");
    let mut cache: LruCache<String, i32> = LruCache::new(3);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    println!("cache = {:?}", cache);
    println!("get a = {:?}", cache.get(&"a".to_string()));
    println!("cache = {:?}", cache);

    println!("\n=== 演示 2：超过容量触发淘汰 ===");
    let mut cache: LruCache<i32, &str> = LruCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");
    println!("插入 1, 2 后: {:?}", cache);
    cache.put(3, "three");        // 容量满，淘汰最久未用的 1
    println!("插入 3 后: {:?}", cache);
    println!("get 1 = {:?}", cache.get(&1));    // None（被淘汰）

    println!("\n=== 演示 3：get 触发 LRU 顺序更新 ===");
    let mut cache: LruCache<char, i32> = LruCache::new(3);
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("初始: {:?}", cache);
    cache.get(&'a');          // 触碰 a，a 变成最新
    cache.put('d', 4);        // 容量满，淘汰 b
    println!("get a + put d 后: {:?}", cache);   // order 应该是 [c, a, d]

    println!("\n=== 演示 4：remove / clear ===");
    let mut cache: LruCache<&str, i32> = LruCache::new(5);
    cache.put("x", 10);
    cache.put("y", 20);
    println!("remove x = {:?}", cache.remove(&"x"));
    println!("after remove: {:?}", cache);
    cache.clear();
    println!("after clear: {:?}", cache);

    println!("\n=== 演示 5：泛型支持任意 Hash + Eq + Clone 的 key ===");
    let mut cache: LruCache<(i32, i32), String> = LruCache::new(3);
    cache.put((1, 2), "1,2".to_string());
    cache.put((3, 4), "3,4".to_string());
    println!("(3, 4) = {:?}", cache.get(&(3, 4)));

    println!("\n=== 演示 6：所有断言 ===");
    run_assertions();
    println!("全部断言通过 ✅");
}

fn run_assertions() {
    // 容量
    let cache: LruCache<i32, i32> = LruCache::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);
    assert!(cache.is_empty());

    // put + get
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), None);

    // 容量满后插入
    cache.put("c", 3);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get(&"a"), None);     // a 被淘汰
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));

    // get 触发 LRU 更新
    let mut cache: LruCache<i32, &str> = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    cache.get(&1);                          // 1 变成最新
    cache.put(4, "four");                   // 淘汰 2
    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&3), Some(&"three"));
    assert_eq!(cache.get(&4), Some(&"four"));

    // 重复 put 不算新条目
    let mut cache: LruCache<i32, &str> = LruCache::new(2);
    cache.put(1, "one");
    cache.put(1, "uno");                    // 覆盖
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&1), Some(&"uno"));

    // remove
    let mut cache: LruCache<i32, i32> = LruCache::new(3);
    cache.put(1, 100);
    cache.put(2, 200);
    assert_eq!(cache.remove(&1), Some(100));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.remove(&1), None);

    // clear
    cache.clear();
    assert!(cache.is_empty());
}
