#include <cstdio>
#include <unistd.h>
#include <sys/types.h>
#include <mpi.h>

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    printf("[pid %d] Hi, my rank is %d\n", getpid(), rank);

    MPI_Finalize();

    return 0;
}
