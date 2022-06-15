# Bury

A tool for burying your ~~secrets~~ files.

## Usage

```shell
$ bury some-file.txt ./a/really/deep/hole/
$ ls ./a/really/deep/hole/
some-text.txt
```

Should follow all of the same behaviors as the `cp` command: https://linuxize.com/post/cp-command-in-linux/

## Installation

```shell
$ cargo install bury-cli
```
