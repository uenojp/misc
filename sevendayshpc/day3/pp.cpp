#include <mpi.h>

#include <cstdio>
#include <cstddef>

void process(int i, int rank) {
    printf("rank: %d index: %d\n", rank, i);
}

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank = -1;
    int nproc = -1;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nproc);

    const size_t n = 100;
    for (size_t i = rank; i < n; i += nproc) {
        process(i, rank);
    }

    MPI_Finalize();
}
