# osbiglab-2025s-2022011223

主要工作：

1. 学习并复现了 Modeled OS 相关工作。
   - 阅读了 ATC'23 [The Hitchhiker's Guide to Operating Systems](https://www.usenix.org/system/files/atc23-jiang-yanyan.pdf) 这篇论文。
   - 以 [MOSAIC](https://github.com/jiangyy/mosaic) 为例研究了相关代码。
   - 从 Modeled OS 出发，尝试为当前 OS 课程的讲义提出建议，辅助理解。
2. 了解了 OS 形式化验证的相关工作。
   - 学习了 Kani 工具的使用，写了一篇学习笔记。
   - 练习：使用 Kani 编写了 [bubble-sort](./kani-exercises/bubble_sort) 的证明。
3. 使用 Kani 验证了 [`memory_addr`](https://github.com/arceos-org/axmm_crates/tree/main/memory_addr) 这个 crate 的正确性。验证的性质包括：
   - 核心地址类型（`MemoryAddr`）的对齐和算术运算。
   - 地址范围（`AddrRange`）的构造和操作的正确性。
   - 页迭代器（`PageIter`）的迭代逻辑和边界条件。
4. 使用 Kani 复现了 [eternalcomet 组](https://github.com/leverimmy/undefined-os-final) 和 [Mivik 组](https://github.com/leverimmy/starry-next-mivik) 的 OS 中的 bug。

每周的进度汇报见 [reports](./reports)。

## Modeled OS

阅读了 ATC'23 [The Hitchhiker's Guide to Operating Systems](https://www.usenix.org/system/files/atc23-jiang-yanyan.pdf) 这篇论文，修改了论文中的 [jiangyy/mosaic](https://github.com/jiangyy/mosaic)，增加了部分 System Call，并增加了 tutorials 部分以适配清华大学计算机系《操作系统》课程的讲义。

- 阅读笔记：[Kani 入门 | Clever_Jimmy's Blog](https://leverimmy.top/2025/04/22/An-Introduction-to-Kani/)。
- 修改后的 MOSAIC 代码位于 [leverimmy/mosaic](https://github.com/leverimmy/mosaic)。
- 修改并补充的《操作系统》讲义的第 1 讲、第 7 讲、第 12 讲的内容，位于 [leverimmy/os-lectures](https://github.com/leverimmy/os-lectures)。

## Verified OS

### Kani

为了探索操作系统形式化验证，我学习了针对 Rust 的形式化验证工具 Kani。Kani 能够通过对（在给定约束内的）所有可能输入进行符号执行，来数学上证明代码的某些性质，或找到违反这些性质的反例。

- 我的学习过程记录在学习笔记中：[Kani 入门 | Clever\_Jimmy's Blog](https://leverimmy.top/2025/04/22/An-Introduction-to-Kani/)。
- 作为一个入门练习，我使用 Kani 编写了对[冒泡排序算法](./kani-exercises/bubble_sort)正确性的证明，并成功让 Kani 找到了一个错误实现版本的反例。

### `verified-memory-addr`

我将 Kani 应用于验证 ArceOS 的一个基础 crate：[`memory_addr`](https://github.com/arceos-org/axmm_crates/tree/main/memory_addr)。通过编写一系列的 Proof Harnesses，我对该 crate 的核心功能进行了形式化验证，确保其在各种边界条件下的行为正确性，代码位于 [leverimmy/verified-memory-addr](https://github.com/leverimmy/verified-memory-addr/)。

主要验证的性质包括：

- **地址对齐与运算**：验证了 `align_up`、`align_down` 和 `is_aligned` 等对齐函数的正确性，确保它们总是返回小于等于或大于等于原地址的、满足对齐要求的最大/最小地址。
- **地址范围 `AddrRange`**：验证了 `new` 的有效性（确保 `start <= end`），以及 `contains`、`overlaps` 等逻辑的正确性。
- **页迭代器 `PageIter`**：验证了页迭代器在处理对齐和非对齐地址时的迭代逻辑、边界条件以及对无效页大小（非 2 的幂）的处理。

通过 Kani 验证了 `memory_addr` crate 中的全部 55 个 Proof Harnesses，确保了其核心功能的正确性。

### 使用 Kani 复现并验证 OS Bug

除了验证独立 crate，我还将 Kani 用于真实操作系统内核的 Bug 分析中，成功对两个小组的 OS Bug 进行了复现和修复验证。

**eternalcomet 组：`sys_execve` 内存泄漏问题**

- **问题**：修复前的 `sys_execve` 在加载完新程序后，会调用一个永不返回的 `enter_uspace` 函数。这导致在 `sys_execve` 栈上创建的一个 `Arc<ProcessData>` 无法被正常 `Drop`，其引用计数无法减一，从而造成内存泄漏。
- **验证**：我通过 Stubbing 隔离被测函数，并引入一个带 `Drop` 标志的 `MonitoredResource`。Kani 成功证明了在旧代码中该资源**永远不会被释放**（`Drop` 不会被调用）。而在修复后的代码（通过修改 `TrapFrame` 正常返回）中，Kani 证明了该资源**总是会被正确释放**。
- **代码**：[leverimmy/undefined-os-final](https://github.com/leverimmy/undefined-os-final/tree/kani)

**Mivik 组：`sys_utimensat` 特殊标志处理错误**

- **问题**：`sys_utimensat` 的原始实现没有正确处理 `timespec` 中的特殊值 `UTIME_NOW`（设置为当前时间）和 `UTIME_OMIT`（不修改时间），而是将它们当作了普通的纳秒值。
- **验证**：通过 Mock `wall_time()` 并用全局变量模拟文件元数据，我编写了 Proof Harness 来检查 `sys_utimensat` 的行为。Kani 成功地为错误实现找到了具体的**反例**，即当输入 `tv_nsec` 为 `UTIME_NOW` 或 `UTIME_OMIT` 的值时，文件的访问和修改时间没有被正确地更新或保持不变，违反了预期的行为。
- **代码**：[leverimmy/starry-next-mivik](https://github.com/leverimmy/starry-next-mivik/tree/kani)
