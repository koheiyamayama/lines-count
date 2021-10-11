# はじめに

このコマンド `lines-count` は引数に取ったファイルの行数を数えるためのコマンドです。

# install
## Mac
```
$ brew update
$ brew search lines-count
$ brew install lines-count
$ lines-count --version
1.0.0
```
## Ubuntu
```
$ apt-get update
$ apt-cache search lines-count
$ apt-get install -y lines-count
$ lines-count --version
1.0.0
```
## From Source
```
$ cargo --version
cargo 1.53.0 (4369396ce 2021-04-27)
$ git clone https://github.com/koheiyamayama/lines-count.git
$ cd /path/to/lines-count
$ cargo build --release
$ ./target/release/lines-count --version
1.0.0
```

# getting started

```shell
$ lines-count /path/to/file
120
$ lines-count /path/to/not-exist-file
argument error: not exist /path/to/not-exist-file
$ lines-count /path/to/directory
220
$ lines-count --extension ts,tsx,rs,yml
{
  "rs": 110,
  "ts": 203,
  "tsx": 512,
  "yml": 21,
  ...
}
$ lines-count --level 0
{
  "./": 1221
}
$ lines-count --level 1
{
  "doc/": 221,
  "src/": 230,
  "target": 510,
  "./": 250,
  ...
}
$ lines-count --level 2
{
  "doc/path1/": 111,
  "doc/path2/": 110,
  "src/": 230,
  "./": 250
}
$ lines-count --level ~
{
  "doc/path1/path3": 111,
  "doc/path1/path4": 111,
  "doc/path1/path4/path5": 111,
  "doc/path2/": 110,
  "src/": 230,
  "./": 250
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


# development progress
- [x] 1ファイルだけ行数を出力できる
- [ ] 1ディレクトリだけ行数を出力できる
- [ ] 拡張子ごとに行数を出力できる
- [ ] ディレクトリごとに行数を出力できる
  - [ ] コマンドラインオプション引数に `~` を指定した場合、全ての最下層のディレクトリごとの行数を出力できる
  - [ ] コマンドラインオプション引数に `数字` を指定した場合、その階層までのディレクトリごとの行数を出力できる
- [ ] --versionオプションを実行できる
- [ ] --helpオプションを実行できる
