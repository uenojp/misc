#include <mpi.h>

#include <cstdio>

void process(const int i, const int rank) {
    printf("rank: %d index: %d\n", rank, i);
}

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank = {};
    int nproc = {};
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nproc);

    const std::size_t n = 100;
    for (std::size_t i = rank; i < n; i += nproc) {
        process(i, rank);
    }

    MPI_Finalize();
}
