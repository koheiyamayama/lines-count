# はじめに

このコマンド `lines-count` は引数に取ったファイルの行数を数えるためのコマンドです。

# getting started

```shell
$ lines-count /path/to/file
120
$ lines-count /path/to/not-exist-file
argument error: not exist /path/to/not-exist-file
$ lines-count /path/to/directory
argument error: use -r option for directory
```
