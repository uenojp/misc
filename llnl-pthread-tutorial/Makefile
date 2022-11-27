CC     = gcc
CFLAGS = -Wall -Wextra
LIBS   = -pthread

SRCS    = $(wildcard *.c)
PROGRAM = $(SRCS:.c=)

all: $(PROGRAM)

%: %.c
	$(CC) $(CFLAGS) -o $@ $< $(LIBS)

clean:
	rm -f $(PROGRAM)

.PHONY: clean
