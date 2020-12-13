# ojichat_rs

[![Crates.io](https://img.shields.io/crates/v/ojichat)](https://crates.io/crates/ojichat)
[![MIT LICENSE](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gamoutatsumi/ojichat_rs/Release)](https://github.com/gamoutatsumi/ojichat_rs/actions?query=workflow:Release)

[ojichat](https://github.com/greymd/ojichat)のRust版実装です。

こちらはライブラリ版になります。

CLIで使いたい方は[こちら](https://github.com/gamoutatsumi/ojichat_rs_cli)

## 開発環境

```bash
$ rustup default
nightly-x86_64-unknown-linux-gnu (default)
```

## 使い方

```rust
use ojichat::ojichat::get_message;

fn main() {
    println!("{}", get_message(None, None, None))
}
```

or

```bash
cargo run --example ojichat
```

```text
オレはチアキちゃんの味方だからネ僕は、すごく心配だよ💦(^▽^;)そんなときは、美味しいもの食べて、元気出さなきゃだね
```

## ライセンス

[MIT](./LICENSE)
