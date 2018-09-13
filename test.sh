#!/bin/bash

# テスト用関数
# 第一引数：テストする値
# 第二引数：期待値
# 第三引数：テスト番号
test() {
    # アセンブリを出力し、gccでcompile
    prog=./target/debug/rcc
    exe=test
    echo "$1" | $prog > ${exe}.s
    gcc ${exe}.s -o $exe
    ./${exe}
    ret=$?
    [ ! $ret -eq "$2" ] && echo "[TestNo.$3] failed (actual $ret but expected $2)"
    rm ${exe}.s ${exe}
}

# ビルドを実行してからテスト
cargo b

test "1+2" 3 1
test "10+20" 30 2
test "100+101" 201 3
test "20-1" 19 4
test "20-10" 10 5
test "20-19" 1 6
test "1+2+3" 6 7
test "1+2-3" 0 8
test "100-2-3" 95 9
test "2*4" 8 10
test "3*4" 12 11
test "5*2*3" 30 12
test "5*2*3-10" 20 13
test "5+2*3" 11 14
test "2*3+3*4" 18 15
test "(12+16)" 28 16
test "(29-16)" 13 17
test "(12+16)+3" 31 18
test "3+(12+16)" 31 19
test "(10+4)*10" 140 20
test "10*(10+4)" 140 21
test "10/5" 2 22
test "20/5/2" 2 23
test "20/3" 6 24
test "2+20/3" 8 25
test "20/3+3" 9 26
test "20%3" 2 27
test "10+20%3" 12 28
test "2==2" 1 29
test "2+2==2*2" 1 30
test "20/10==2" 1 31
test "1==2" 0 32
test "2!=2" 0 33
test "2+2!=2*2" 0 34
test "20/10!=2" 0 35
test "1!=2" 1 36
#test 256 256 # 255までしか数値を扱うことができない

