#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <strings.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
    char buf[256];
    ssize_t nread;
    int flags;

    flags = fcntl(STDIN_FILENO, F_GETFL, 0);
    fcntl(STDIN_FILENO, F_SETFL, flags|O_NONBLOCK);

    memset(buf, 0, sizeof(buf));

    for(;;) {
        nread = read(STDIN_FILENO, buf, sizeof(buf));
        if (nread == -1) {
            if (errno == EINTR) {
                continue;
            }
            if (errno == EAGAIN) {
                fprintf(stderr, "wait 3s and read it again\n");
                sleep(3);
                continue;
            }
            perror("read");
        }

        printf("buf: %s\n", buf);
    }

    return 0;
}
