CC     = g++
CFLAGS = -Wall -Wextra
LIBS   = -lbfd

PROGRAM = loader_demo
SRCS    = $(wildcard *.cc)
OBJS    = $(SRCS:.cc=.o)

$(PROGRAM): $(OBJS)
	$(CC) -o $(PROGRAM) $(OBJS) $(LIBS)

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $<

clean:
	rm -f $(PROGRAM) $(OBJS)

.PHONY: clean
