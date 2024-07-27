#!/usr/bin/env -S gnuplot -p
set grid
set logscale
set xlabel "len"
set ylabel "time[us]"

plot "O0_idiot" using 1:3 with lines,\
     "O3_idiot" using 1:3 with lines,\
     "strcpy"   using 1:3 with lines,\
     "memcpy"   using 1:3 with lines
