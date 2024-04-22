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

```shell
cargo run -- csv -i .\assets\juventus.csv
```

## 生成随机密码

```shell
# 生成长度为 100 的随机密码
cargo run -- genpass -l 100
```

## 文本验签

```shell
# 生成ed25519格式的key
cargo run -- text gen -d ./fixtures -f ed25519
# 生成blake3格式的key
cargo run -- text gen -d ./fixtures -f

# 使用 blake3算法， 根据 ./fixtures/blake3.txt 文件里面的key, 计算 ./fixtures/blake3_input_test.txt 文件的 签名信息，签名信息在控制台
cargo run -- text sign -k ./fixtures/blake3.txt -i ./fixtures/blake3_input_test.txt

# 使用 blake3算法, 根据 ./fixtures/blake3.txt 文件里面的key, 计算 ./fixtures/blake3_input_test.txt 文件的 签名信息，判断与给定的签名信息是否匹配
cargo run -- text verify -k ./fixtures/blake3.txt -i ./fixtures/blake3_input_test.txt -s "nVJo82CLfGBz24iG4rOJz_rQ-4fCdhplXWKZPfFiNkw"

# 使用 ed25519 算法， 根据 ./fixtures/ed25519.sk 文件里面的key, 计算  ./fixtures/ed25519_input_test.txt 文件的 签名信息，签名信息在控制台
cargo run -- text sign -k ./fixtures/ed25519.sk -i ./fixtures/ed25519_input_test.txt -f ed25519

# 使用 ed25519 算法, 根据 ./fixtures/ed25519.pk 文件里面的key, 计算  ./fixtures/ed25519_input_test.txt 文件的 签名信息，判断与给定的签名信息是否匹配
cargo run -- text verify -k ./fixtures/ed25519.pk -i ./fixtures/ed25519_input_test.txt -f ed25519 -s "qiohbnmjC1VmCs-XuwSn_uKTqkbScYzaAbeEOGC8rdpMNPNwoFU3hq6FMwOomiZ6_7cFzNz1vfcVZ2vr03GaBg"
```
