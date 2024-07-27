
`make run` result at Intel(R) Core(TM) i5-10400F (L3 cache: 12 MiB (1 instance))

| run | L3 miss rate |
| --- | --- |
| ./eviction 1048576      |     0.0104 |
| ./eviction 4194304      |     0.0138 |
| ./eviction 8388608      |     0.0214 |
| ./eviction 16777216     |     0.0337 |
| ./no-eviction 1048576   |     0.0003 |
| ./no-eviction 4194304   |     0.0019 |
| ./no-eviction 8388608   |     0.0182 |
| ./no-eviction 16777216  |     0.0345 |

Enviroment
```bash
❯ uname -a
Linux Home 6.5.0-45-generic #45~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Mon Jul 15 16:40:02 UTC 2 x86_64 x86_64 x86_64 GNU/Linux
❯ gcc --version
gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0
Copyright (C) 2021 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
❯ likwid-perfctr --version
likwid-perfctr -- Version 5.3.0 (commit: d686eabcde3bb046b9061aac5325dd0ded009e8e)
```


FULL LOG:
```
# NOTE:
# L3 cache: 12 MiB @ Intel(R) Core(TM) i5-10400F
likwid-perfctr -C 0 -g L3CACHE -m ./eviction 1048576
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
62.649367 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.008043 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |   18392310 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |   33455020 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |   23697370 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |     879263 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |     227819 |
|     UOPS_RETIRED_ALL     |   PMC2  |   21940090 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0080 |
| Runtime unhalted [s] |     0.0115 |
|      Clock [MHz]     |  4099.5020 |
|          CPI         |     1.8190 |
|    L3 request rate   |     0.0505 |
|     L3 miss rate     |     0.0104 |
|     L3 miss ratio    |     0.2058 |
+----------------------+------------+

likwid-perfctr -C 0 -g L3CACHE -m ./eviction 4194304
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
263.866961 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.033810 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |   71915850 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |  137327100 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |   95605250 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |    3464182 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |    1176890 |
|     UOPS_RETIRED_ALL     |   PMC2  |   85200080 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0338 |
| Runtime unhalted [s] |     0.0473 |
|      Clock [MHz]     |  4171.2841 |
|          CPI         |     1.9096 |
|    L3 request rate   |     0.0545 |
|     L3 miss rate     |     0.0138 |
|     L3 miss ratio    |     0.2536 |
+----------------------+------------+

likwid-perfctr -C 0 -g L3CACHE -m ./eviction 8388608
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
675.993781 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.086553 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |  143254100 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |  354296400 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |  242705900 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |    6614742 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |    3632450 |
|     UOPS_RETIRED_ALL     |   PMC2  |  169533000 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0866 |
| Runtime unhalted [s] |     0.1220 |
|      Clock [MHz]     |  4239.1975 |
|          CPI         |     2.4732 |
|    L3 request rate   |     0.0604 |
|     L3 miss rate     |     0.0214 |
|     L3 miss ratio    |     0.3545 |
+----------------------+------------+

################## L3 cache boundary ################
likwid-perfctr -C 0 -g L3CACHE -m ./eviction 16777216
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
1669.695570 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.213759 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |  285896600 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |  851607200 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |  606308500 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |   11357360 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |   11381380 |
|     UOPS_RETIRED_ALL     |   PMC2  |  338088200 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.2138 |
| Runtime unhalted [s] |     0.2933 |
|      Clock [MHz]     |  4078.6863 |
|          CPI         |     2.9787 |
|    L3 request rate   |     0.0673 |
|     L3 miss rate     |     0.0337 |
|     L3 miss ratio    |     0.5005 |
+----------------------+------------+


######################################################

likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 1048576
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
46.363305 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.005946 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |   18380930 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |   21487230 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |   17496000 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |    1131754 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |       6268 |
|     UOPS_RETIRED_ALL     |   PMC2  |   21857480 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0059 |
| Runtime unhalted [s] |     0.0074 |
|      Clock [MHz]     |  3566.4689 |
|          CPI         |     1.1690 |
|    L3 request rate   |     0.0521 |
|     L3 miss rate     |     0.0003 |
|     L3 miss ratio    |     0.0055 |
+----------------------+------------+

likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 4194304
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
173.872086 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.022275 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |   71870400 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |   92066560 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |   62213720 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |    4174482 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |     164968 |
|     UOPS_RETIRED_ALL     |   PMC2  |   85040240 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0223 |
| Runtime unhalted [s] |     0.0317 |
|      Clock [MHz]     |  4297.4321 |
|          CPI         |     1.2810 |
|    L3 request rate   |     0.0510 |
|     L3 miss rate     |     0.0019 |
|     L3 miss ratio    |     0.0380 |
+----------------------+------------+

likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 8388608
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
603.363930 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.077258 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |  143220000 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |  300977500 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |  214654000 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |    6744977 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |    3081416 |
|     UOPS_RETIRED_ALL     |   PMC2  |  169461800 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.0773 |
| Runtime unhalted [s] |     0.1036 |
|      Clock [MHz]     |  4071.8401 |
|          CPI         |     2.1015 |
|    L3 request rate   |     0.0580 |
|     L3 miss rate     |     0.0182 |
|     L3 miss ratio    |     0.3136 |
+----------------------+------------+

################## L3 cache boundary ################
likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 16777216
--------------------------------------------------------------------------------
CPU name:	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz
CPU type:	Intel Cometlake processor
CPU clock:	2.90 GHz
--------------------------------------------------------------------------------
1759.096242 us
--------------------------------------------------------------------------------
Region strcpy, Group 1: L3CACHE
+-------------------+------------+
|    Region Info    | HWThread 0 |
+-------------------+------------+
| RDTSC Runtime [s] |   0.225198 |
|     call count    |        128 |
+-------------------+------------+

+--------------------------+---------+------------+
|           Event          | Counter | HWThread 0 |
+--------------------------+---------+------------+
|     INSTR_RETIRED_ANY    |  FIXC0  |  285896600 |
|   CPU_CLK_UNHALTED_CORE  |  FIXC1  |  900777300 |
|   CPU_CLK_UNHALTED_REF   |  FIXC2  |  632695700 |
|  MEM_LOAD_RETIRED_L3_HIT |   PMC0  |   10537990 |
| MEM_LOAD_RETIRED_L3_MISS |   PMC1  |   11670960 |
|     UOPS_RETIRED_ALL     |   PMC2  |  338088300 |
+--------------------------+---------+------------+

+----------------------+------------+
|        Metric        | HWThread 0 |
+----------------------+------------+
|  Runtime (RDTSC) [s] |     0.2252 |
| Runtime unhalted [s] |     0.3102 |
|      Clock [MHz]     |  4134.4611 |
|          CPI         |     3.1507 |
|    L3 request rate   |     0.0657 |
|     L3 miss rate     |     0.0345 |
|     L3 miss ratio    |     0.5255 |
+----------------------+------------+
```
