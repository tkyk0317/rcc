#!/bin/bash

# テスト用関数
# 第一引数：テストする値
# 第二引数：期待値
test() {
    # アセンブリを出力し、gccでcompile
    prog=./target/debug/rcc
    exe=test
    echo "$1" | $prog > ${exe}.s
    gcc ${exe}.s -o $exe
    ./${exe}
    ret=$?
    [ ! $ret -eq "$2" ] && echo "$exe failed (actual $ret but expected $2)"
    rm ${exe}.s ${exe}
}

test 29 29
test 128 128
test 255 255
#test 256 256 # 255までしか数値を扱うことができない

