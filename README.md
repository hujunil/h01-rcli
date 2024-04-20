# 通过 CLI 项目巩固 Rust 语言基础

## 环境配置

解决 cargo deny 的配置问题。

在项目下面初始化配置 `cargo deny init`, 在项目目录下会生成 `deny.toml` 文件。修改一些核心的配置如下：

```toml
[advisories]
db-path = "$CARGO_HOME/advisory-dbs"
db-urls = ["https://github.com/rustsec/advisory-db"]
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "Unicode-DFS-2016",
]
[sources.allow-org]
github = []
gitlab = []
bitbucket = []
```

在项目下面执行 `cargo deny check`, 根据错误提示信息作出对应的处理。

## 解析 csv

```
cargo run -- csv -i .\assets\juventus.csv
```
