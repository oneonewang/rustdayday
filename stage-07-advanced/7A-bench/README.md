# Stage 7A · 性能基准

> 难度：⭐⭐  
> 综合运用：criterion + rayon + black_box  
> 预计时间：1 – 2 小时

## 🎯 项目目标

用 **criterion** 给一组常见操作做基准测试，对比：
- 单线程 vs rayon 并行
- naive vs 优化（埃氏筛、矩阵转置）
- 不同数据结构（HashMap vs Vec）

## 📂 项目结构

```
stage-07-advanced/7A-bench/
├── Cargo.toml
├── README.md
├── src/lib.rs              # 待测函数实现
└── benches/benches.rs      # criterion 基准
```

## 🚀 怎么跑

```bash
cd stage-07-advanced/7A-bench
cargo bench                  # 跑全部 benchmark
cargo bench --bench sum      # 只跑 sum
cargo bench -- --save-baseline main   # 保存基线
cargo bench -- --baseline main        # 对比基线
```

报告输出在 `target/criterion/`（HTML 报告可双击看图）。

## 📚 涉及的基准

| 名称 | 对比 |
|------|------|
| `sum_1M` | serial vs rayon::par_iter |
| `sort_100k` | std::sort vs rayon::par_sort |
| `primes_100k` | naive serial / naive parallel / sieve |
| `string_upper_100x` | to_uppercase vs ascii_only |
| `word_count` | HashMap vs fold |
| `matmul_50x50` | naive vs transposed |
| `fib_30` | recursive vs iterative |

## 🧪 典型结果（仅供参考）

```
sum_1M/serial     time:   [2.4 ms ...]
sum_1M/parallel   time:   [1.1 ms]    ← rayon 在 1M 上快 2x

primes_100k/serial    time:   [38 ms]
primes_100k/parallel  time:   [9 ms]    ← 4x
primes_100k/sieve     time:   [3 ms]    ← 算法比并行更重要！

matmul_50x50/naive       time:   [800 µs]
matmul_50x50/transposed  time:   [500 µs]  ← 缓存友好
```

> **关键洞察**：**算法 > 并行**。在写并行版本之前先看算法能不能优化。

## 🎁 扩展挑战

1. **Criterion 报告比较**：跑两次 `cargo bench` 比较输出
2. **perf + flamegraph**：`cargo install flamegraph` → `cargo flamegraph --bench benches`
3. **加更大输入**：把 N 调大观察不同实现的"拐点"
4. **加内存分配基准**：用 `dhat` / `heaptrack` 看哪个实现更省内存

## 📚 关键 crate

- `criterion`：统计稳健的基准（自动 warmup、多次采样、置信区间）
- `criterion-plot`：绘图后端
- `rayon`：并行迭代器
- `black_box`：阻止编译器优化掉待测代码

## ✅ 完成判定

- [ ] `cargo bench` 跑通
- [ ] 至少 3 组基准有结论（serial vs parallel 哪个快、naive vs 优化哪个快）
- [ ] 看懂 criterion 报告（HTML 里的 mean / median / slope）
- [ ] 至少完成 1 个扩展挑战

完成 → 告诉我开始 7B / 7C / 7D 中的哪个。
