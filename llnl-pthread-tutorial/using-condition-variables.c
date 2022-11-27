#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

const size_t nthread = 10;

const uint64_t count_limit = 12345;
const count_per_thread = 5000;

uint64_t count = 0;

pthread_mutex_t count_mutex;
pthread_cond_t count_threashold_cond;

void* increment_count(void* thread_id) {
    for (size_t i = 0; i < count_per_thread; i++) {
        pthread_mutex_lock(&count_mutex);

        count++;
        if (count == count_limit) {
            fprintf(stderr, "%ld\n", count);
            pthread_cond_signal(&count_threashold_cond);
        }

        pthread_mutex_unlock(&count_mutex);
    }
    pthread_exit(NULL);
}

void* wait_count(void* thread_id) {
    pthread_mutex_lock(&count_mutex);
    while (count < count_limit) {
        pthread_cond_wait(&count_threashold_cond, &count_mutex);
    }
    pthread_mutex_unlock(&count_mutex);
    pthread_exit(NULL);
}

int main(void) {
    pthread_t threads[nthread];

    pthread_mutex_init(&count_mutex, NULL);

    if (pthread_create(&threads[0], NULL, wait_count, (void*)0) != 0) {
        perror("pthread_create");
    }
    for (size_t i = 1; i < nthread; i++) {
        if (pthread_create(&threads[i], NULL, increment_count, (void*)i) != 0) {
            perror("pthread_create");
        }
    }

    for (size_t i = 0; i < nthread; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("pthread_join");
        }
    }

    pthread_mutex_destroy(&count_mutex);

    printf("%ld\n", count);

    return 0;
}
