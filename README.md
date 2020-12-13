# ojichat_rs

[ojichat](https://github.com/greymd/ojichat)のRust版実装です。

こちらはライブラリ版になります。

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
