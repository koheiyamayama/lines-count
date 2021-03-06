# はじめに

このコマンド `lines-count` は引数に取ったファイルの行数を数えるためのコマンドです。

# install
## Mac
```
$ brew update
$ brew tap koheiyamayama/lines-count
$ brew search lines-count
$ brew install lines-count
```
## Ubuntu(progress)
```
$ apt-get update
$ apt-cache search lines-count
$ apt-get install -y lines-count
```
## From Source
```
$ cargo --version
cargo 1.53.0 (4369396ce 2021-04-27)
$ git clone https://github.com/koheiyamayama/lines-count.git
$ cd /path/to/lines-count
$ cargo build --release
$ ./target/release/lines-count --version
```

# getting started

```shell
$ lines-count /path/to/file
120
$ lines-count /path/to/not-exist-file
argument error: not exist /path/to/not-exist-file
$ lines-count /path/to/directory
220
$ tree
.
├── sample.rs
├── sample.ts
├── sample_one
│   ├── sample.py
│   └── sample.rb
└── sample_second
    ├── sample.py
    ├── sample.rb
    └── sample_third
        └── sample.ex
$ lines-count --extension
{
  "rs": 3,
  "rb": 6,
  "py": 20,
  "txt": 1,
  "ex": 15,
}
$ lines-count --level 0
{
  "./": 45
}
$ lines-count --level 1
{
  "./": 4,
  "./sample_one": 13,
  "./sample_second": 28,
}
$ lines-count --level 2
{
  "./": 4,
  "./sample_one": 13,
  "./sample_second": 13,
  "./sample_third": 15,
}
$ lines-count --level ~
{
  "./": 4,
  "./sample_one": 13,
  "./sample_second": 13,
  "./sample_third": 15,
  "./sample_fourth": 2,
}
```

# 引数
- ファイルパス
- ディレクトリパス

# オプション
- --extensions
  - ファイル拡張子ごとに行数を出力する。
- --level n | ~
  - n
    - 何かしらの数字である。
    - 何階層分のディレクトリまで深堀りし、ディレクトリごとに行数を出力する。
  - ~
    - 一番下のディレクトリまで深堀りし、ディレクトリごとに行数を出力する。
- --excludes /path/to/file,/path/to/directory,...
  - 出力結果に含めないファイル名、ディレクトリ名をカンマ区切りで渡す。
- --includes /path/to/file,/path/to/directory,...
  - 出力結果に含めるファイル名、ディレクトリ名をカンマ区切りで渡す。


# development progress
- [x] 1ファイルだけ行数を出力できる
- [x] 1ディレクトリだけ行数を出力できる
- [x] 拡張子ごとに行数を出力できる
- [ ] ディレクトリごとに行数を出力できる
  - [ ] コマンドラインオプション引数に `~` を指定した場合、全ての最下層のディレクトリごとの行数を出力できる
  - [ ] コマンドラインオプション引数に `数字` を指定した場合、その階層までのディレクトリごとの行数を出力できる
- [x] --versionオプションを実行できる
- [x] --helpオプションを実行できる
