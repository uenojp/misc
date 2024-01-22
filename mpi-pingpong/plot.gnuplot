#!/bin/env gnuplot

set terminal pdf
set output 'bandwidth.pdf'

set grid x y
set xrange [1:]
set yrange [0:]
set logscale x

set xlabel "Data Size"
set ylabel "Bandwidth (GB/s)"
set format x "1x10^{%T}"

plot "bandwidth.dat" using 1:2 with linespoints title 'Bandwidth'

