thermal: thermal.exe
	mkdir $@ || true
	./$<

mpi-thermal: mpi-thermal.exe
	mkdir $@ || true
	mpirun -np 4 -oversubscribe ./$<

gather: gather.exe
	mpirun -np 4 -oversubscribe ./$<

%.exe: %.cpp
	mpic++ -o $@ $<

.PHONY: clean
clean:
	$(RM) thermal.exe mpi-thermal.exe gather.exe
	$(RM) -r thermal/ mpi-thermal/

