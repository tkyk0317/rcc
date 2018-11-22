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

test "main() { 1+2; }" 3 1
test "main() { 10+20; }" 30 2
test "main() { 100+101; }" 201 3
test "main() { 20-1; }" 19 4
test "main() { 20-10; }" 10 5
test "main() { 20-19; }" 1 6
test "main() { 1+2+3; }" 6 7
test "main() { 1+2-3; }" 0 8
test "main() { 100-2-3; }" 95 9
test "main() { 2*4; }" 8 10
test "main() { 3*4; }" 12 11
test "main() { 5*2*3; }" 30 12
test "main() { 5*2*3-10; }" 20 13
test "main() { 5+2*3; }" 11 14
test "main() { 2*3+3*4; }" 18 15
test "main() { (12+16); }" 28 16
test "main() { (29-16); }" 13 17
test "main() { (12+16)+3; }" 31 18
test "main() { 3+(12+16); }" 31 19
test "main() { (10+4)*10; }" 140 20
test "main() { 10*(10+4); }" 140 21
test "main() { 10/5; }" 2 22
test "main() { 20/5/2; }" 2 23
test "main() { 20/3; }" 6 24
test "main() { 2+20/3; }" 8 25
test "main() { 20/3+3; }" 9 26
test "main() { 20%3; }" 2 27
test "main() { 10+20%3; }" 12 28
test "main() { 2==2; }" 1 29
test "main() { 2+2==2*2; }" 1 30
test "main() { 20/10==2; }" 1 31
test "main() { 1==2; }" 0 32
test "main() { 2!=2; }" 0 33
test "main() { 2+2!=2*2; }" 0 34
test "main() { 20/10!=2; }" 0 35
test "main() { 1!=2; }" 1 36
test "main() { 1>2; }" 0 37
test "main() { 1<2; }" 1 38
test "main() { 1+3-1>2*4; }" 0 39
test "main() { 1*3+20>4*2/2; }" 1 40
test "main() { 1>=2; }" 0 41
test "main() { 1>=1; }" 1 42
test "main() { 2>=1; }" 1 43
test "main() { 1<=2; }" 1 44
test "main() { 2<=2; }" 1 45
test "main() { 2<=3; }" 1 46
test "main() { 1+3-1>=2*4; }" 0 47
test "main() { 1+3-1+5>=2*4; }" 1 48
test "main() { 1*3+20>=4*2/2; }" 1 49
test "main() { 1*3+20>=23; }" 1 50
test "main() { 1+3-1<=2*4; }" 1 51
test "main() { 1+3-1+5<=2*4; }" 1 52
test "main() { 1*3+20<=4*2/2; }" 0 53
test "main() { 1*3+20<=23; }" 1 54
test "main() { 3*(2+2) >= 4+(3*1); }" 1 55
test "main() { 1&&1; }" 1 56
test "main() { 0&&1; }" 0 57
test "main() { (1 + 1) && (2 * 1); }" 1 58
test "main() { 1 == 1 && 2 < 1; }" 0 59
test "main() { 4 / 2 == 0 + 2 && 2 > 1; }" 1 60
test "main() { 1||1; }" 1 61
test "main() { 0||0; }" 0 62
test "main() { (1 + 1) || (2 * 1); }" 1 63
test "main() { 1 != 1 || 2 < 1; }" 0 64
test "main() { 4 / 2 == 0 + 2 || 2 < 1; }" 1 65
test "main() { (1 == 0 && 1) && (2 < 1 || 0); }" 0 66
test "main() { 2 ? 1 : 3; }" 1 67
test "main() { 2 > 1 ? 1 : 3; }" 1 68
test "main() { 2 < 1 ? 1 : 3; }" 3 69
test "main() { 2 > 1 ? (2 ? 10 : 100) : 3; }" 10 70
test "main() { 2 == 1 ? (2 == 2 ? 9 : 99) : (0 ? 10 : 100); }" 100 71
test "main() { +2; }" 2 72
test "main() { 5 + (-5); }" 0 73
test "main() { 3 - + - + - + - 2; }" 5 73
test "main() { !2; }" 0 74
test "main() { !(2 + 2 == 3 * 4); }" 1 75
test "main() { !(2 != 3); }" 0 76
test "main() { 2<<1; }" 4 77
test "main() { 2>>1; }" 1 78
test "main() { 2<<1<<1; }" 8 79
test "main() { 8>>1>>1; }" 2 80
test "main() { 2<<3; }" 16 81
test "main() { 16>>2; }" 4 82
test "main() { 5>>1; }" 2 83
test "main() { 1&1; }" 1 84
test "main() { 1&0; }" 0 85
test "main() { 1|0; }" 1 86
test "main() { 1|1; }" 1 87
test "main() { 1^1; }" 0 88
test "main() { 0^1; }" 1 89
test "main() { 0^0; }" 0 90
test "main() { 1&0|1; }" 1 91
test "main() { 183&109; }" 37 92
test "main() { 183|109; }" 255 93
test "main() { 183^109; }" 218 94
test "main() { ~183 & 255; }" 72 96
test "main() { 2+2; 1+2; }" 3 97
test "main() { 5>>1; 1 != 2; }" 1 98
test "main() { x = 4; x = x * x + 1; x + 3; }" 20 99
test "main() { x = 2 * 3 * 4; }" 24 100
test "main() { x = x = x = 3; }" 3 101
test "test() { 1; } main() { test(); }" 1 102
test "test() { a = 1;  a + 19;} main() { test(); }" 20 103
test "test() { a = 1; } main() { test(); 10; }" 10 104
test "test(a) { a + 1; } main() { test(1); }" 2 105
test "test(a) { a = a * 2; a + 10; } main() { b = 10; test(b); }" 30 106
test "main() { if (10 == 10) { a = 2; a * 9; } }" 18 107
test "main() { if (10 != 10) { a = 2; a * 9; } 2; }" 2 108
test "main() { if (2 == 10) { a = 2; a * 9; } 11; }" 11 109
test "main() { if (1 != 10) { a = 3; a + 9; } }" 12 110
test "main() { if (1 == 10) { 9; } else { 4; } }" 4 111
test "main() { a = 0; while (a < 1) { a = a + 1; } a; }" 1 112
test "main() { a = 0; while (a < 2) { a = a + 1; } a; }" 2 113
test "main() { a = 0; while (a <= 2) { a = a + 1; } a; }" 3 114
test "main() { a = 8; b = 1; a = a + b; a; }" 9 115
test "main() { for (i = 0 ; i < 2 ; i = i + 1) {;} 11; }" 11 116
test "main() { a = 0; for (i = 0 ; i < 10 ; i = i + 1) { a = a + 1;} a; }" 10 117
test "test(a, b) { a + b; } main() { test(1, 4); }" 5 118
#test 256 256 # 255までしか数値を扱うことができない

