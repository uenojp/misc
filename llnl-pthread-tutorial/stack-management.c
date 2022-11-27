#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

const size_t N = 10000;
const size_t MEGA = 1000 * 10000;

pthread_attr_t attr;

void* worker(void* thread_id) {
    size_t tid = (size_t)thread_id;

    size_t stacksize;
    pthread_attr_getstacksize(&attr, &stacksize);
    fprintf(stderr, "worker(tid %ld): stack size %ld\n", tid, stacksize);

    double A[N][N];
    for (size_t i = 0; i < N; i++) {
        for (size_t j = 0; j < N; j++) {
            A[i][j] = (i * j);
        }
    }

    pthread_exit(NULL);
}

int main(void) {
    const size_t nthreads = 10;
    pthread_t threads[nthreads];

    size_t stacksize;
    pthread_attr_init(&attr);
    pthread_attr_getstacksize(&attr, &stacksize);
    fprintf(stderr, "main: Default stack size %ld\n", stacksize);

    stacksize = sizeof(double) * N * N + MEGA;
    pthread_attr_setstacksize(&attr, stacksize);
    fprintf(stderr, "main: Creating threads with stack size %ld\n", stacksize);
    for (size_t i = 0; i < nthreads; i++) {
        if (pthread_create(&threads[i], &attr, worker, (void*)i) != 0) {
            perror("main: pthread_create");
            exit(1);
        }
    }

    pthread_exit(NULL);
}
