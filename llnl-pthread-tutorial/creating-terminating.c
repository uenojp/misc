#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

void* print_hello(void* thread_id) {
    printf("thread_id: %ld\n", (size_t)thread_id);

    pthread_exit(NULL);
}

int main(void) {
    const size_t n = 5;

    pthread_t threads[n];
    for (size_t i = 0; i < n; i++) {
        if (pthread_create(&threads[i], NULL, print_hello, (void*)i) != 0) {
            perror("pthread_create");
            exit(1);
        }
    }

    pthread_exit(NULL);
}
