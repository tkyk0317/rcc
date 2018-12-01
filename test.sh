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

test "int main() { return 1+2; }" 3 1
test "int main() { return 10+20; }" 30 2
test "int main() { return 100+101; }" 201 3
test "int main() { return 20-1; }" 19 4
test "int main() { return 20-10; }" 10 5
test "int main() { return 20-19; }" 1 6
test "int main() { return 1+2+3; }" 6 7
test "int main() { return 1+2-3; }" 0 8
test "int main() { return 100-2-3; }" 95 9
test "int main() { return 2*4; }" 8 10
test "int main() { return 3*4; }" 12 11
test "int main() { return 5*2*3; }" 30 12
test "int main() { return 5*2*3-10; }" 20 13
test "int main() { return 5+2*3; }" 11 14
test "int main() { return 2*3+3*4; }" 18 15
test "int main() { return (12+16); }" 28 16
test "int main() { return (29-16); }" 13 17
test "int main() { return (12+16)+3; }" 31 18
test "int main() { return 3+(12+16); }" 31 19
test "int main() { return (10+4)*10; }" 140 20
test "int main() { return 10*(10+4); }" 140 21
test "int main() { return 10/5; }" 2 22
test "int main() { return 20/5/2; }" 2 23
test "int main() { return 20/3; }" 6 24
test "int main() { return 2+20/3; }" 8 25
test "int main() { return 20/3+3; }" 9 26
test "int main() { return 20%3; }" 2 27
test "int main() { return 10+20%3; }" 12 28
test "int main() { return 2==2; }" 1 29
test "int main() { return 2+2==2*2; }" 1 30
test "int main() { return 20/10==2; }" 1 31
test "int main() { return 1==2; }" 0 32
test "int main() { return 2!=2; }" 0 33
test "int main() { return 2+2!=2*2; }" 0 34
test "int main() { return 20/10!=2; }" 0 35
test "int main() { return 1!=2; }" 1 36
test "int main() { return 1>2; }" 0 37
test "int main() { return 1<2; }" 1 38
test "int main() { return 1+3-1>2*4; }" 0 39
test "int main() { return 1*3+20>4*2/2; }" 1 40
test "int main() { return 1>=2; }" 0 41
test "int main() { return 1>=1; }" 1 42
test "int main() { return 2>=1; }" 1 43
test "int main() { return 1<=2; }" 1 44
test "int main() { return 2<=2; }" 1 45
test "int main() { return 2<=3; }" 1 46
test "int main() { return 1+3-1>=2*4; }" 0 47
test "int main() { return 1+3-1+5>=2*4; }" 1 48
test "int main() { return 1*3+20>=4*2/2; }" 1 49
test "int main() { return 1*3+20>=23; }" 1 50
test "int main() { return 1+3-1<=2*4; }" 1 51
test "int main() { return 1+3-1+5<=2*4; }" 1 52
test "int main() { return 1*3+20<=4*2/2; }" 0 53
test "int main() { return 1*3+20<=23; }" 1 54
test "int main() { return 3*(2+2) >= 4+(3*1); }" 1 55
test "int main() { return 1&&1; }" 1 56
test "int main() { return 0&&1; }" 0 57
test "int main() { return (1 + 1) && (2 * 1); }" 1 58
test "int main() { return 1 == 1 && 2 < 1; }" 0 59
test "int main() { return 4 / 2 == 0 + 2 && 2 > 1; }" 1 60
test "int main() { return 1||1; }" 1 61
test "int main() { return 0||0; }" 0 62
test "int main() { return (1 + 1) || (2 * 1); }" 1 63
test "int main() { return 1 != 1 || 2 < 1; }" 0 64
test "int main() { return 4 / 2 == 0 + 2 || 2 < 1; }" 1 65
test "int main() { return (1 == 0 && 1) && (2 < 1 || 0); }" 0 66
test "int main() { return 2 ? 1 : 3; }" 1 67
test "int main() { return 2 > 1 ? 1 : 3; }" 1 68
test "int main() { return 2 < 1 ? 1 : 3; }" 3 69
test "int main() { return 2 > 1 ? (2 ? 10 : 100) : 3; }" 10 70
test "int main() { return 2 == 1 ? (2 == 2 ? 9 : 99) : (0 ? 10 : 100); }" 100 71
test "int main() { return +2; }" 2 72
test "int main() { return 5 + (-5); }" 0 73
test "int main() { return 3 - + - + - + - 2; }" 5 73
test "int main() { return !2; }" 0 74
test "int main() { return !(2 + 2 == 3 * 4); }" 1 75
test "int main() { return !(2 != 3); }" 0 76
test "int main() { return 2<<1; }" 4 77
test "int main() { return 2>>1; }" 1 78
test "int main() { return 2<<1<<1; }" 8 79
test "int main() { return 8>>1>>1; }" 2 80
test "int main() { return 2<<3; }" 16 81
test "int main() { return 16>>2; }" 4 82
test "int main() { return 5>>1; }" 2 83
test "int main() { return 1&1; }" 1 84
test "int main() { return 1&0; }" 0 85
test "int main() { return 1|0; }" 1 86
test "int main() { return 1|1; }" 1 87
test "int main() { return 1^1; }" 0 88
test "int main() { return 0^1; }" 1 89
test "int main() { return 0^0; }" 0 90
test "int main() { return 1&0|1; }" 1 91
test "int main() { return 183&109; }" 37 92
test "int main() { return 183|109; }" 255 93
test "int main() { return 183^109; }" 218 94
test "int main() { return ~183 & 255; }" 72 96
test "int main() { 2+2; return 1+2; }" 3 97
test "int main() { 5>>1; return 1 != 2; }" 1 98
test "int main() { int x; x = 4; x = x * x + 1; x = x + 3; return x; }" 20 99
test "int main() { int x; x = 2 * 3 * 4; return x; }" 24 100
test "int main() { int x; x = x = x = 3; return x; }" 3 101
test "int test() { return 1; } int main() { return test(); }" 1 102
test "int test() { int a; a = 1; return a + 19;} int main() { return test(); }" 20 103
test "int test() { return 1; } int main() { test(); return 10; }" 10 104
test "int test(int a) { return a + 1; } int main() { return test(1); }" 2 105
test "int test(int a) { a = a * 2; return a + 10; } int main() { int b; b = 10; return test(b); }" 30 106
test "int main() { int a; a = 0; if (10 == 10) { a = 2; a = a * 9; } return a; }" 18 107
test "int main() { if (10 != 10) { int a; a = 2; a * 9; } return 2; }" 2 108
test "int main() { if (2 == 10) { int a; a = 2; a * 9; } return 11; }" 11 109
test "int main() { int a; a = 0; if (1 != 10) { a = 3; a = a + 9; } return a; }" 12 110
test "int main() { if (1 == 10) { return 9; } else { return 4; } }" 4 111
test "int main() { int a; a = 0; while (a < 1) { a = a + 1; } return a; }" 1 112
test "int main() { int a; a = 0; while (a < 2) { a = a + 1; } return a; }" 2 113
test "int main() { int a; a = 0; while (a <= 2) { a = a + 1; } return a; }" 3 114
test "int main() { int a; a = 8; int b; b = 1; a = a + b; return a; }" 9 115
test "int main() { int i; i = 0; for (i = 0 ; i < 2 ; i = i + 1) {;} return 11; }" 11 116
test "int main() { int a; a = 0; int i; i = 0; for (i = 0 ; i < 10 ; i = i + 1) { a = a + 1;} return a; }" 10 117
test "int test(int a, int b) { return a + b; } int main() { return test(1, 4); }" 5 118
test "int main() { int a; a = 0; do { a = a + 1; } while (a <= 2); return a; }" 3 119
test "int main() { int i; i = 0; while (1) { i = i + 1; if (i < 100) { continue; } else { break; } } return i; }" 100 121
test "int main() { int i; i = 0; do { i = i + 1; if (i < 100) { continue; } else { break; } } while(1); return i; }" 100 122
test "int main() { int i; i = 0; for (;; i = i + 1) { if (i < 100) { continue; } else { break; } } return i; }" 100 123
test "int main() { return 1; }" 1 124
test "int main() { return 1 + 2; }" 3 125
test "int main() { int a; a = 100; return a; }" 100 126
test "int main() { return 1 == 4; }" 0 127
test "int main() { int a; a = 0; if (1 == 10) a = 9; else a = 4; return a; }" 4 128
test "int main() { if (1 != 10) return 1; else return 10; }" 1 129

#test 256 256 # 255までしか数値を扱うことができない

