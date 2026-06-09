# 06 · Stage 4 复习与综合自测

> **本章目标**：把 Stage 4 串成图，做 10 道综合题。**通过再进 Stage 5。**

## 6.1 知识地图

```
Stage 4 概念图
════════════════════════════════════════════════════════════════

   智能指针                 闭包                迭代器
   ────────                 ────                ────
   Box<T>                   |x| x + 1           trait Iterator { next }
   Rc<T>                    Fn / FnMut / FnOnce 惰性求值
   Arc<T>                   move 关键字         组合子
   RefCell<T>               捕获方式            collect / sum / fold
   Arc<Mutex<T>>            impl Fn             自定义迭代器
   Weak<T>                                      ExactSizeIterator
   Cow<'a, T>                                   DoubleEndedIterator
   Pin<P<T>>
       │                        │                    │
       │      共享 + 可变        │   函数式风格        │   流式处理
       └────────────────────────┴────────────────────┘
                                │
                                ▼
                  ┌────────────────────────┐
                  │  内部可变性             │
                  │  编译期 → 运行时检查    │
                  │  RefCell / Cell        │
                  │  Mutex / RwLock        │
                  └────────────────────────┘
```

## 6.2 一句话回顾

- **智能指针** = 包装堆数据，控制"谁拥有、谁能改、是否线程安全"。
- **闭包** = 能捕获环境的匿名函数；三 trait 控制"消耗 / 借用 / 可改"。
- **迭代器** = 惰性 + 零分配 + 链式；实现 `next` 一个方法就拿到 70+ 适配器。
- **内部可变性** = 借用检查从编译期推迟到运行时。
- **`Arc<Mutex<T>>`** = 跨线程共享 + 可变的标准模式。

## 6.3 综合自测（10 题）

### 题 1：递归 enum + Box

定义 `IntList { Cons(i32, Box<IntList>), Nil }`，实现 `sum` / `len` / `to_vec`（不递归太深爆栈）。

### 题 2：Rc 共享链表

用 `Rc` 写一个链表，演示"prepend 不消耗 self"。

### 题 3：RefCell 缓存

写一个"懒缓存"：第一次调 `get_or_init(f)` 算并存值，之后直接返回。**不要**用 `OnceCell`——用 `RefCell<Option<T>>` 手写。

### 题 4：闭包 trait 选择

为以下每个闭包写正确的 trait bound（`Fn` / `FnMut` / `FnOnce`）：

```rust
let s = String::from("hi");
let a = || println!("{s}");             // ?
let mut n = 0;
let b = || n += 1;                      // ?
let v = vec![1, 2, 3];
let c = || v.into_iter().sum::<i32>();  // ?
```

### 题 5：实现自定义 Iterator

写一个 `FibIter { curr: u64, next: u64 }`，产出斐波那契数列。用 `take(10).collect::<Vec<_>>()` 验证前 10 项。

### 题 6：filter_map 一遍搞定

把 `[Some(1), None, Some(2), None, Some(3)]` 转成 `[1, 2, 3]`，**用一行** `filter_map` 链。

### 题 7：Arc<Mutex<T>> 多线程计数

10 个线程并发对共享计数器 `+1`，最终结果 = 10。

### 题 8：Weak 父子节点

写一个"父强引子、子弱引父"的树节点，验证父 drop 后子能正确降级。

### 题 9：Cow 避免不必要分配

写 `normalize(s: &str) -> Cow<str>`：如果首尾有空白返回 `Owned(trim)`，否则 `Borrowed(s)`。测试两种情况。

### 题 10：综合：CSV 第一列提取

写 `first_column(text: &str) -> Vec<&str>`，用迭代器链实现（split → filter → map → collect），**不要**显式 `for` 循环。

## 6.4 答案要点

| 题 | 关键点 |
|----|--------|
| 1 | `match self { Cons(h, t) => h + t.sum(), Nil => 0 }` |
| 2 | `prepend(&self, ...)` 返回新 List，`Rc::clone(&self.head)` 共享 |
| 3 | `RefCell<Option<T>>`，`get_or_init` 用 `borrow` / `borrow_mut` |
| 4 | a=Fn, b=FnMut, c=FnOnce |
| 5 | `next` 累加 curr/next，return curr |
| 6 | `[1,2,3,4,5].into_iter().filter_map(|x| if x % 2 == 0 { Some(x*10) } else { None }).collect()` 类比 |
| 7 | `Arc<Mutex<i32>>` + `lock().unwrap() += 1` |
| 8 | `parent: Weak<Self>` + `upgrade()` |
| 9 | `if s.trim().len() == s.len() { Cow::Borrowed(s) } else { Cow::Owned(s.trim().into()) }` |
| 10 | `text.lines().filter(|l| !l.is_empty()).map(|l| l.split(',').next().unwrap()).collect()` |

## 6.5 通过标准

- 10 题中 **8 题** 在 60 分钟内一次写对
- 所有 [`exercises/`](./../exercises) 和 [`project-04-text-parser/`](./../project-04-text-parser) 能 `cargo build` 通过

## 6.6 阶段回顾清单

> 进入 Stage 5 之前自问：

- [ ] `Box<T>` 三种用途（堆分配 / 递归 / DST）
- [ ] `Rc` vs `Arc` 的取舍
- [ ] `RefCell` 内部可变性的本质
- [ ] 闭包三种 trait 的区别
- [ ] `move` 关键字什么时候必须
- [ ] `Iterator` trait 的核心方法
- [ ] 惰性求值的好处
- [ ] 写自定义 `Iterator`
- [ ] `Arc<Mutex<T>>` 跨线程共享模式
- [ ] `Weak` 打破循环
- [ ] `Cow` 用法和适用场景

某条不确定就回去重读。

## 6.7 推荐复习间隔

> - 写完 1 天后做 6.3 自测
> - 进入 Stage 5 之前再做一遍
> - 1 个月后再做（迭代器、自定义 Iterator 是后期常用工具）

---

🎉 Stage 4 完！准备好后告诉我开始 Stage 5（并发与异步——Tokio、async/await、Send/Sync）。
