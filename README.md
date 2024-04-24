# 通过 CLI 项目巩固 Rust 语言基础

## 环境配置

配置 pre-commit

```shell
pre-commit install
pre-commit run --all-files
```

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
cargo run -- text verify -k ./fixtures/blake3.txt -i ./fixtures/blake3_input_test.txt -s "d_AUMpqi9tpUVJURQZcfBrscwtN2UyHxNCBW61WfFk0"

# 使用 ed25519 算法， 根据 ./fixtures/ed25519.sk 文件里面的key, 计算  ./fixtures/ed25519_input_test.txt 文件的 签名信息，签名信息在控制台
cargo run -- text sign -k ./fixtures/ed25519.sk -i ./fixtures/ed25519_input_test.txt -f ed25519

# 使用 ed25519 算法, 根据 ./fixtures/ed25519.pk 文件里面的key, 计算  ./fixtures/ed25519_input_test.txt 文件的 签名信息，判断与给定的签名信息是否匹配
cargo run -- text verify -k ./fixtures/ed25519.pk -i ./fixtures/ed25519_input_test.txt -f ed25519 -s "yt47pU6h8Bdhj92DAJzJvwQi5dCJziNOCp_3ORX3CL-J_52NUojRYAWPv7OQLWR4qx07ruQnsl4XXBSz2DyACA"
```

## http 静态文件服务

```shell
# 启动服务
cargo run -- http serve -d . -p 9090

curl http://127.0.0.1:9090/fixtures/index.html
curl curl http://127.0.0.1:9090/tower/fixtures/index.html
```

# 作业

## 作业 1

```shell
# 加密
cargo run -- text encrypt --key "password" -i <(echo -n "helloworld")

# 解密
cargo run -- text decrypt --key "password" -i <(echo -n "Im/VMgx1EVxRTaBdSf5JmsrkUAlqMgTkbDNeziOUMynq1S6tTNw=")
```

## 作业 2

```shell
# 生成 jwt 签名信息， Audience 指定为 tester, developer, leader 中的一个
cargo run -- jwt sign --sub "user" --aud "tester" --exp "10d"

# 验证签名
cargo run -- jwt verify --token "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJ0ZXN0ZXIiLCJleHAiOjE3MTQ4MTI0MzEsInN1YiI6InVzZXIifQ.jfvUuXxvEsG2ZVZ7smSecFfTIekmLsn_qfgSv8Uitfs"

```

## 作业 3

```shell
# 启动服务
cargo run -- http serve -d .

# 浏览器访问
http://127.0.0.1:8080/fixtures
```
