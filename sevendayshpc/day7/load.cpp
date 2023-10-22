#include <x86intrin.h>

#include <cstdio>

void printm256d(__m256d x) {
    printf("%f %f %f %f\n", x[3], x[2], x[1], x[0]);
}

__attribute__((aligned(32))) double a[] = {0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0};

int main(void) {
    __m256d x = _mm256_load_pd(a);
    __m256d y = _mm256_load_pd(a + 4);
    printm256d(x + y);
}
