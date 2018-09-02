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

test 1+2 3
test 10+20 30
test 100+101 201
test 20-1 19
test 20-10 10
test 20-19 1
test 1+2+3 6
test 1+2-3 0
test 100-2-3 95
#test 256 256 # 255までしか数値を扱うことができない

