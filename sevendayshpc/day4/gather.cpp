#include <mpi.h>

#include <cstdio>
#include <vector>

const int L = 31;

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank;
    int nprocess;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nprocess);

    const int size = L / nprocess;

    std::vector<int> local(size, rank);
    std::vector<int> global(L, -1);

    MPI_Gather(local.data(), size, MPI_INT, global.data(), size, MPI_INT, 0, MPI_COMM_WORLD);

    if (rank == 0) {
        for (auto& e : global) {
            printf("%d,", e);
        }
        puts("");
    }

    MPI_Finalize();
}
