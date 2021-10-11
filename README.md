# はじめに

このコマンド `lines-count` は引数に取ったファイルの行数を数えるためのコマンドです。

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

