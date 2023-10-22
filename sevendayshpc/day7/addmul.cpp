#include <x86intrin.h>

#include <cstdio>

void printm256d(__m256d x) {
    printf("%f %f %f %f\n", x[3], x[2], x[1], x[0]);
}

int main(void) {
    __m256d a = _mm256_set_pd(3.0, 2.0, 1.0, 0.0);
    __m256d b = _mm256_set_pd(0.0, 1.0, 2.0, 3.0);

    printm256d(a + b);
    printm256d(a * b);

    return 0;
}
