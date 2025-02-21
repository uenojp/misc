set terminal pngcairo size 1600,1200
set output "zennendougetsu.png"

set multiplot layout 4,4

set grid
set offset 0, 0, 0.5, 0


set label 1 "自動スケール: グラフの概形を見る" at screen 0.5, 0.75 center
set label 2 "スケールを合わせる: 値の相対的な比較をする" at screen 0.5, 0.25 center

#
# 自動スケール
#
set ylabel "なんらかの指数"
plot "seq-inc.dat" with lp title "seq-inc: raw data"
unset ylabel
plot "exp-inc.dat" with lp title "exp-inc: raw data"
plot "seq-dec.dat" with lp title "seq-dec: raw data"
plot "exp-dec.dat" with lp title "exp-dec: raw data"

set xlabel "月数"; set ylabel "なんらかの指数の前年同月比"
set yrange [0:]
plot "seq-inc-zennendougetsu.dat" with lp title "seq-inc: 前年同月比", 1 lc "black" notitle
unset ylabel
plot "exp-inc-zennendougetsu.dat" with lp title "exp-inc: 前年同月比", 1 lc "black" notitle
plot "seq-dec-zennendougetsu.dat" with lp title "seq-dec: 前年同月比", 1 lc "black" notitle
plot "exp-dec-zennendougetsu.dat" with lp title "exp-dec: 前年同月比", 1 lc "black" notitle
unset xlabel


#
# スケールを合わせる
#
set linetype 1 lc rgb "#ff0000" # 1番目の線を赤

set yrange [0:3000]; set ylabel "なんらかの指数"
plot "seq-inc.dat" with lp title "seq-inc: raw data"
unset ylabel
plot "exp-inc.dat" with lp title "exp-inc: raw data"
plot "seq-dec.dat" with lp title "seq-dec: raw data"
plot "exp-dec.dat" with lp title "exp-dec: raw data"

set yrange [0:4]; set xlabel "月数"; set ylabel "なんらかの指数の前年同月比"
plot "seq-inc-zennendougetsu.dat" with lp title "seq-inc: 前年同月比", 1 lc "black" notitle
unset ylabel
plot "exp-inc-zennendougetsu.dat" with lp title "exp-inc: 前年同月比", 1 lc "black" notitle
plot "seq-dec-zennendougetsu.dat" with lp title "seq-dec: 前年同月比", 1 lc "black" notitle
plot "exp-dec-zennendougetsu.dat" with lp title "exp-dec: 前年同月比", 1 lc "black" notitle

unset multiplot
set output

