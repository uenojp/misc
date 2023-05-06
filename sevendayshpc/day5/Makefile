CC     = g++
CFLAGS = -Wall -Wextra -O3
LIBS   =

PROGRAM = gray-scott
SRCS    = $(wildcard *.cpp)
OBJS    = $(SRCS:.cpp=.o)

gen: $(PROGRAM)
	@mkdir data/
	./$(PROGRAM)
	./image.rb
	(cd data && convert -delay 5 conf*png conf.gif)

$(PROGRAM): $(OBJS)
	$(CC) -o $(PROGRAM) $(OBJS) $(LIBS)

%.o: %.cpp
	$(CC) $(CFLAGS) -o $@ -c $<

clean:
	rm -f $(PROGRAM) $(OBJS)
	rm -rf data/

.PHONY: clean
