.PHONY: all run clean
all: eviction no-eviction
run: eviction no-eviction
	# NOTE:
	# L3 cache: 12 MiB @ Intel(R) Core(TM) i5-10400F
	likwid-perfctr -C 0 -g L3CACHE -m ./eviction 1048576
	likwid-perfctr -C 0 -g L3CACHE -m ./eviction 4194304
	likwid-perfctr -C 0 -g L3CACHE -m ./eviction 8388608
	################## L3 cache boundary ################
	likwid-perfctr -C 0 -g L3CACHE -m ./eviction 16777216
	@echo
	@echo '######################################################'
	@echo
	likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 1048576
	likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 4194304
	likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 8388608
	################## L3 cache boundary ################
	likwid-perfctr -C 0 -g L3CACHE -m ./no-eviction 16777216
clean:; rm eviction no-eviction

eviction: main.c
	gcc -O3 -DLIKWID_PERFMON -DEVICTION -o $@ $< -llikwid
no-eviction: main.c
	gcc -O3 -DLIKWID_PERFMON -o $@ $< -llikwid
