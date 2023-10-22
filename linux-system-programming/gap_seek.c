#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

int main(void) {
    int fd;

    fd = open("test", O_CREAT | O_RDWR, S_IRUSR | S_IWUSR);
    // filled with '\0' for 1M
    lseek(fd, 1 * 1024 * 1024 - 1, SEEK_SET);
    write(fd, "\0", 1);

    close(fd);
}
