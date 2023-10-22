#include <bits/posix1_lim.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <limits.h>

int main(void) {
    ssize_t nread;
    char buf[2048] = {};
    char *p;
    size_t len;
    int fd;

    if ((fd = open("test", O_RDONLY)) == -1) {
        perror("open");
        return 1;
    }

    len = sizeof(buf);
    p = (char *)buf;

    if (len > SSIZE_MAX) {
        len = SSIZE_MAX;
    }
    while (len != 0 && (nread = read(fd, p, len)) != 0) {
        if (nread == -1) {
            if (errno == EINTR) {
                continue;
            }
            perror("read");
            break;
        }
        len -= nread;
        p += nread;
    }

    printf("%s", buf);

    return 0;
}
