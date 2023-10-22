#include <stdio.h>
#include <sys/select.h>
#include <sys/time.h>
#include <sys/types.h>
#include <unistd.h>

int main(void) {
    struct timeval tv;
    fd_set readfds;
    int ret;

    FD_ZERO(&readfds);
    FD_SET(STDIN_FILENO, &readfds);

    tv.tv_sec = 5;
    tv.tv_usec = 0;

    if ((ret = select(STDIN_FILENO + 1, &readfds, NULL, NULL, &tv)) == -1) {
        perror("select");
        return 1;
    }
    if (!ret) {
        printf("timeout: %d seconds\n", 5);
        return 0;
    }

    if (FD_ISSET(STDIN_FILENO, &readfds)) {
        char buf[2024] = {};
        ssize_t nread;

        if ((nread = read(STDIN_FILENO, buf, sizeof(buf))) == -1) {
            perror("read");
            return 1;
        }

        if (nread) {
            buf[nread] = '\0';
            printf("remain %lds%ldus read: %s", tv.tv_sec, tv.tv_usec, buf);
        }

        return 0;
    }

    fprintf(stderr, "unreachable\n");

    return -1;
}
