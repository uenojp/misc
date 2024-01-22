#!/bin/env gnuplot

set terminal pdf
set output 'bandwidth.pdf'

set grid x y
set xrange [1:]
set yrange [0:]
set logscale x

set xlabel "Data Size (Byte)"
set ylabel "Bandwidth (MiB/s)"
set format x "1x10^{%T}"

plot "bandwidth.dat" using 1:3 with linespoints title 'Bandwidth'

