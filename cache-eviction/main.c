#include <stdio.h>
#include <string.h>
#include <time.h>
#include <stdlib.h>

#include <likwid.h>
#include <likwid-marker.h>

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s length\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    const size_t length = atol(argv[1]);
    char* src = malloc(length);
    char* dest = malloc(length);
    // for cache eviction
    // Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
    const size_t l3_cach_size = 12 * (1 << 20) * 8;  // 12MiB
    const size_t cache_line_size = 64 * 8;           // 64B
    char* dummy = malloc(l3_cach_size);
    if (!src || !dest || !dummy) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(src, 1, length);
    src[length - 1] = 0;

    const size_t repetitions = 128;
    double elapse_us = 0.0;
    struct timespec start, end;

    LIKWID_MARKER_INIT;

    for (size_t i = 0; i < repetitions; i++) {
#ifdef EVICTION
        // evict src and dest data in cache
        for (size_t offset = 0; offset < l3_cach_size; offset += cache_line_size) {
            dummy[offset] = 127;
        }
#endif

        LIKWID_MARKER_START("strcpy");
        clock_gettime(CLOCK_MONOTONIC, &start);
        strcpy(dest, src);
        clock_gettime(CLOCK_MONOTONIC, &end);
        LIKWID_MARKER_STOP("strcpy");
        elapse_us += (double)(end.tv_sec - start.tv_sec) * 1.0e6 +
            (double)(end.tv_nsec - start.tv_nsec) * 1.0e-3;
    }
    elapse_us /= (double)repetitions;

    LIKWID_MARKER_CLOSE;

    printf("%lf us\n", elapse_us);

    return 0;
}
