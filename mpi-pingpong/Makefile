CC		= mpic++
CFLAGS  = -O3

PROGRAM	= pingpong

$(PROGRAM): $(PROGRAM).c

bandwidth.dat: $(PROGRAM)
	mpirun -np 2 ./$(PROGRAM) > $@

plot: bandwidth.dat plot.gnuplot
	gnuplot plot.gnuplot

clean:
	$(RM) $(PROGRAM) bandwidth.dat bandwidth.pdf

.PHONY: plot clean

