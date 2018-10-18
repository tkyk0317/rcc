#include <stdio.h>

int test_func(void) {
  printf("function for test\n");
  return 0;
}

int func_arg1(int a) {
    printf("%d\n", a);
    return 0;
}

int func_arg2(int a, int b) {
    printf("%d\n", a + b);
    return 0;
}

int func_arg3(int a, int b, int c) {
    printf("%d, %d, %d\n", a, b, c);
    return 0;
}

int func_arg4(int a, int b, int c, int d) {
    printf("%d\n", a * b * c * d);
    return 0;
}

int func_arg5(int a, int b, int c, int d, int e) {
    printf("%d, %d, %d, %d, %d\n", a, b, c ,d ,e);
    return 0;
}

int func_arg6(int a, int b, int c, int d, int e, int f) {
    printf("%d\n", a + b + c + d + e + f);
    return 0;
}
