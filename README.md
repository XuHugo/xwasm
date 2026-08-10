# xwasm

xwasm 是一个面向智能合约执行的 Rust MVP：使用 eDSL 编写合约并编译为 Wasm，通过 Wasmtime 执行，再由 Block-STM 对区块交易进行推测并行执行，最后将确定的结果原子提交到 StateDB。

当前目标不是提供完整区块链节点，而是验证下面这条高性能合约执行链路能够正确闭环：

~~~mermaid
flowchart LR
    A["Rust eDSL 合约"] --> B["wasm32 合约"]
    B --> C["Wasmtime AOT 模块"]
    C --> D["Block-STM 并行执行"]
    D --> E["串行语义校验"]
    E --> F["StateDB 原子提交"]
    F --> G["重启恢复状态"]
~~~

## MVP 状态

当前功能型 MVP 已经完成：

- eDSL 合约可以编译为 wasm32 合约。
- Wasmtime host functions 支持合约参数、上下文和状态读写。
- Block-STM 支持按交易顺序进行多版本读写、验证、回滚和重执行。
- 串行与并行执行会比较完整输出，保证确定性。
- 成功交易的写集合通过 LevelDB WriteBatch 原子提交。
- 数据库关闭并重新打开后可以恢复合约状态。
- Wasmtime Engine、epoch ticker 和 Module 已复用或缓存。
- 交易共享 AOT Wasm 字节，避免逐笔复制和重复预编译。
- MVP 自适应并发上限固定为 4。
- 已覆盖单热点 Key、复杂多 Key 依赖和动态写集合变化。

## 快速开始

### 环境要求

- Rust 与 Cargo
- rustup
- wasm32-unknown-unknown target
- Linux 或 WSL 环境

第一次运行时，脚本会自动安装 wasm32 target，并构建 Release 合约。首次构建还需要下载 Rust 依赖。

### 运行端到端 MVP

在仓库根目录执行：

~~~bash
bash scripts/run-mvp.sh
~~~

这个命令会：

1. 将 contract eDSL 合约编译为 Release Wasm。
2. 使用当前 Wasm 执行并行交易。
3. 校验交易执行成功。
4. 将输出原子提交到临时 StateDB。
5. 关闭并重新打开数据库。
6. 验证恢复后的 last_call 状态。

## Demo 合约

contract/src/lib.rs 当前包含以下入口：

| 合约入口 | 用途 | 状态访问特征 |
| --- | --- | --- |
| xq_init | 初始化及参数解析 | 无持久化写入 |
| xq_abc | 保存最后一次调用 | 所有交易写 last_call |
| xq_store | 按 name 保存状态 | 不同 name 形成独立 Key |
| xq_increment | 递增共享计数器 | 读写热点 counter |
| xq_compute | CPU 密集计算 | 无状态冲突 |

这些入口分别用于验证基础执行、独立 Key、热点冲突和计算密集型并行收益。

## 性能测试

### 使用方法

~~~bash
bash scripts/bench-mvp.sh [transactions] [concurrency] [mode]
~~~

默认参数：

- transactions：100
- concurrency：4
- mode：all

mode 支持：

| 模式 | 合约入口 | 说明 |
| --- | --- | --- |
| independent | xq_store | 每笔交易写不同 Key |
| hotspot | xq_increment | 所有交易读写同一个 counter |
| compute | xq_compute | 每笔交易执行 200,000 次计算循环 |
| all | 上述全部 | 依次运行三类负载 |

例如，使用 4 并发运行 500 笔独立 Key 交易：

~~~bash
bash scripts/bench-mvp.sh 500 4 independent
~~~

benchmark 会输出：

- 串行与并行耗时
- 串行与并行 TPS
- 加速比
- speculative abort 次数
- 平均单笔执行时间
- 冲突率
- 推荐并发度

### 当前参考结果

以下结果来自当前开发机器的 Release 构建，每类负载预热后运行 7 轮并取中位数。它用于判断实现方向，不是跨机器性能承诺。

| 负载 | 交易数 | 串行 TPS | 4 并发 TPS | 加速比 | 中位回滚数 | 推荐并发 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 独立 Key | 500 | 7,932 | 4,813 | 0.574x | 0 | 1 |
| 热点 counter | 500 | 9,147 | 2,743 | 0.331x | 404 | 1 |
| 计算密集 | 100 | 1,170 | 3,151 | 2.667x | 0 | 4 |

结论：

- 计算足够重且冲突低时，4 并发能够获得明确收益。
- 便宜交易即使没有 Key 冲突，也可能因为调度和 Wasm 实例开销而比串行慢。
- 高冲突交易会产生大量推测回滚，不适合强制并行。
- 并发度不是越高越好，MVP 当前以 4 为经过验证的上限。

## 自适应并发策略

recommended_concurrency 根据区块大小、平均交易成本和冲突率返回 1、2 或 4：

1. 交易数少于 32，或者平均交易成本低于 250 微秒：使用 1。
2. 冲突率达到 20%：最多使用 2。
3. 冲突率达到 5%：最多使用 4。
4. 其他重型低冲突负载：使用调用方上限，但 MVP 最多为 4。

当前 ExecutionProfile 由调用方提供。在线采样、滚动统计和自动反馈控制尚未包含在 MVP 中。

## 测试

运行 par-wasm 完整测试：

~~~bash
cargo test --manifest-path par-wasm/Cargo.toml --all-targets
~~~

当前 par-wasm 共 13 项测试，覆盖：

- eDSL Wasm 基础执行和结果顺序
- 交易内 read-your-own-write
- 单热点 Key 的重执行
- 热点区块的串行/并行一致性
- A → B → C 三级 Key 依赖
- A/B/C 交叉冲突和重叠读写集合
- 重执行后写集合从 B 切换到 C，并清除旧推测写
- concurrency=1 的串行回退
- 成功输出的原子提交、last-write-wins 和删除
- 真实 eDSL Wasm 提交及数据库重启恢复
- 自适应并发策略

复杂冲突测试会对串行和 4 并发的状态、写集合、事件、Gas 与执行结果逐项比较。动态写集合测试还会断言确实发生 speculative abort，并已连续运行 20 次验证稳定性。

## 项目结构

| 目录 | 作用 |
| --- | --- |
| contract | 示例 eDSL 合约，编译目标为 wasm32 |
| xq-derive | eDSL 过程宏 |
| xq-std | 合约侧 Context 与标准接口 |
| xq-wasm | Wasm ABI 和辅助实现 |
| wasm | 原始串行 Wasmtime 执行实现 |
| par-wasm | 当前 MVP 的串行/并行 Wasm Runtime、测试和 benchmark |
| parallel | Block-STM 调度器、多版本状态和执行器 |
| statedb | LevelDB 状态存储及原子批量提交 |
| types | 交易、状态 Key、读写集合等共享类型 |
| scripts | 一键 Demo 与 benchmark 脚本 |
| vendor | MVP 使用的本地依赖修复 |

各 crate 当前拥有独立 Cargo.toml，仓库根目录不是 Cargo workspace，因此命令需要通过 --manifest-path 指定 crate。

## 当前边界

这个仓库目前证明的是合约执行 MVP，而不是生产级区块链节点。尚未覆盖：

- 网络、共识、mempool 和完整区块生命周期
- 持续在线的冲突率/交易成本采样
- 大规模随机化差分和长时间 soak test
- 跨机器、固定 CPU 亲和性和统计显著性的正式性能报告
- 生产级资源配额、审计和安全加固
- 高于 4 worker 的稳定调优与容量模型

下一阶段建议优先增加 CI、随机多 Key 差分测试、长期性能基线和真实业务合约负载。
