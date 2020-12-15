//! [ojichat](https://github.com/greymd/ojichat)のRust版実装です。
//!
//! こちらはライブラリ版になります。
//!
//! CLIで使いたい方は[こちら](https://github.com/gamoutatsumi/ojichat_rs_cli)
//!
//! # 開発環境
//!
//! ```bash
//! $ rustup default
//! nightly-x86_64-unknown-linux-gnu (default)
//! ```
//!
//! # 使い方
//!
//! ```rust
//! use ojichat::ojichat::get_message;
//!
//! fn print_message(
//!     target: Option<String>,
//!     emoji_num: Option<usize>,
//!     punctuation_level: Option<usize>,
//! ) {
//!     println!("{}", get_message(target, emoji_num, punctuation_level))
//! }
//! ```
//!
//! or
//!
//! ```bash
//! cargo run --example ojichat
//! ```
//!
//! > オレはチアキちゃんの味方だからネ僕は、すごく心配だよ💦(^▽^;)そんなときは、美味しいもの食べて、元気出さなきゃだね

mod generator;
mod pattern;

pub mod ojichat {
    use super::generator::Generator;
    pub fn get_message(
        target: Option<String>,
        emoji_num: Option<usize>,
        punctuation_level: Option<usize>,
    ) -> String {
        let emoji_num: usize = emoji_num.unwrap_or(4);
        let punctuation_level: usize = punctuation_level.unwrap_or(0);
        Generator::get_message(target, emoji_num, punctuation_level)
    }
}
