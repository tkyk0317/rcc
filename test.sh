#!/bin/bash
# Macかどうかを判定し、環境変数設定.
if [ "$(uname)" == 'Darwin' ]; then
    export TARGET="mac"
fi

# テスト用関数

# 生成したプログラムの出力が期待値通りになっているか、確認.
#
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

# 関数コール用テスト関数
# 第一引数：関数コール文
# 第二引数：スタブ
# 第三引数：期待値
test_call_func() {
    exp=$1
    stub=$2
    output=$3
    prog=./target/debug/rcc

    echo "$exp" | $prog > out.s
    if [ $? -ne 0 ]; then
        echo generate assembly error from "$exp"
        rm out.s
        exit 1
    fi

    gcc -c out.s -o out.o
    if [ $? -ne 0 ]; then
        echo generate object file from from "$exp"
        cat out.s
        rm out.s
        exit 1
    fi

    gcc -c "$stub" -o stub.o
    if [ $? -ne 0 ]; then
        echo compile error "$stub"
        rm out.s out.o
        exit 1
    fi

    gcc out.o stub.o -o out
    if [ $? -ne 0 ]; then
        echo link error from "$exp"
        cat out.s
        rm out.s out.o stub.o
        exit 1
    fi

    ./out > stdout.txt
    ret=$?

    if [ $ret -ne 0 ]; then
        echo "$exp" should be 0, but got $ret.
        cat out.s
        rm out.s out.o stub.o out
        exit 1
    fi

    echo "$output" | diff - stdout.txt > /dev/null
    if [ $? -ne 0 ]; then
        echo expect stdout to be \""$output"\", but got \""$(cat stdout.txt)"\".
        cat out.s
        rm out.s out.o stub.o out stdout.txt
        exit 1
    fi

    rm out.s out.o stub.o out stdout.txt
}

# ビルドを実行してからテスト
cargo b

test "1+2;" 3 1
test "10+20;" 30 2
test "100+101;" 201 3
test "20-1;" 19 4
test "20-10;" 10 5
test "20-19;" 1 6
test "1+2+3;" 6 7
test "1+2-3;" 0 8
test "100-2-3;" 95 9
test "2*4;" 8 10
test "3*4;" 12 11
test "5*2*3;" 30 12
test "5*2*3-10;" 20 13
test "5+2*3;" 11 14
test "2*3+3*4;" 18 15
test "(12+16);" 28 16
test "(29-16);" 13 17
test "(12+16)+3;" 31 18
test "3+(12+16);" 31 19
test "(10+4)*10;" 140 20
test "10*(10+4);" 140 21
test "10/5;" 2 22
test "20/5/2;" 2 23
test "20/3;" 6 24
test "2+20/3;" 8 25
test "20/3+3;" 9 26
test "20%3;" 2 27
test "10+20%3;" 12 28
test "2==2;" 1 29
test "2+2==2*2;" 1 30
test "20/10==2;" 1 31
test "1==2;" 0 32
test "2!=2;" 0 33
test "2+2!=2*2;" 0 34
test "20/10!=2;" 0 35
test "1!=2;" 1 36
test "1>2;" 0 37
test "1<2;" 1 38
test "1+3-1>2*4;" 0 39
test "1*3+20>4*2/2;" 1 40
test "1>=2;" 0 41
test "1>=1;" 1 42
test "2>=1;" 1 43
test "1<=2;" 1 44
test "2<=2;" 1 45
test "2<=3;" 1 46
test "1+3-1>=2*4;" 0 47
test "1+3-1+5>=2*4;" 1 48
test "1*3+20>=4*2/2;" 1 49
test "1*3+20>=23;" 1 50
test "1+3-1<=2*4;" 1 51
test "1+3-1+5<=2*4;" 1 52
test "1*3+20<=4*2/2;" 0 53
test "1*3+20<=23;" 1 54
test "3*(2+2) >= 4+(3*1);" 1 55
test "1&&1;" 1 56
test "0&&1;" 0 57
test "(1 + 1) && (2 * 1);" 1 58
test "1 == 1 && 2 < 1;" 0 59
test "4 / 2 == 0 + 2 && 2 > 1;" 1 60
test "1||1;" 1 61
test "0||0;" 0 62
test "(1 + 1) || (2 * 1);" 1 63
test "1 != 1 || 2 < 1;" 0 64
test "4 / 2 == 0 + 2 || 2 < 1;" 1 65
test "(1 == 0 && 1) && (2 < 1 || 0);" 0 66
test "2 ? 1 : 3;" 1 67
test "2 > 1 ? 1 : 3;" 1 68
test "2 < 1 ? 1 : 3;" 3 69
test "2 > 1 ? (2 ? 10 : 100) : 3;" 10 70
test "2 == 1 ? (2 == 2 ? 9 : 99) : (0 ? 10 : 100);" 100 71
test "+2;" 2 72
test "5 + (-5);" 0 73
test  "3 - + - + - + - 2;" 5 73
test "!2;" 0 74
test "!(2 + 2 == 3 * 4);" 1 75
test "!(2 != 3);" 0 76
test "2<<1;" 4 77
test "2>>1;" 1 78
test "2<<1<<1;" 8 79
test "8>>1>>1;" 2 80
test "2<<3;" 16 81
test "16>>2;" 4 82
test "5>>1;" 2 83
test "1&1;" 1 84
test "1&0;" 0 85
test "1|0;" 1 86
test "1|1;" 1 87
test "1^1;" 0 88
test "0^1;" 1 89
test "0^0;" 0 90
test "1&0|1;" 1 91
test "183&109;" 37 92
test "183|109;" 255 93
test "183^109;" 218 94
test "~183 & 255;" 72 96
test "2+2; 1+2;" 3 97
test "5>>1; 1 != 2" 1 98
test "x = 3; x = x * x + 1; x + 3;" 13 100
test "x = 2 * 3 * 4;" 24 101
test "x = x = x = 3;" 3 102
#test 256 256 # 255までしか数値を扱うことができない

# 関数コールテスト.
test_call_func "test_func();" "./tests/stub.c" "function for test"

