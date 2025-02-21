#!/bin/bash

# 24ヶ月間で線形に増加
seq 1 24 | awk '{print 300+10*$1}' > seq-inc.dat
paste <(head -n 12 seq-inc.dat) <(tail -n 12 seq-inc.dat) | awk '{printf "%.3f\n", $2/$1}' > seq-inc-zennendougetsu.dat

# 24ヶ月間で指数的に増加
seq 1 24 | awk '{printf "%.3f\n", 300*1.1^$1}' > exp-inc.dat
paste <(head -n 12 exp-inc.dat) <(tail -n 12 exp-inc.dat) | awk '{printf "%.3f\n", $2/$1}' > exp-inc-zennendougetsu.dat


# 24ヶ月間で線形に減少
seq 1 24 | awk '{print 300-10*$1}' > seq-dec.dat
paste <(head -n 12 seq-dec.dat) <(tail -n 12 seq-dec.dat) | awk '{printf "%.3f\n", $2/$1}' > seq-dec-zennendougetsu.dat

# 24ヶ月間で指数的に減少
seq 1 24 | awk '{printf "%.3f\n", 300*0.9^$1}' > exp-dec.dat
paste <(head -n 12 exp-dec.dat) <(tail -n 12 exp-dec.dat) | awk '{printf "%.3f\n", $2/$1}' > exp-dec-zennendougetsu.dat

