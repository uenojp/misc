#include <mpi.h>

#include <cstdio>
#include <random>

const size_t n = 1024 * 1024;

double calculate_pi(const int seed) {
    std::mt19937 mt(seed);
    std::uniform_real_distribution<double> ud(0.0, 1.0);

    std::size_t m = 0;

    for (std::size_t i = 0; i < n; i++) {
        auto x = ud(mt);
        auto y = ud(mt);
        if (x * x + y * y <= 1.0) {
            m += 1;
        }
    }

    return 4.0 * static_cast<double>(m) / static_cast<double>(n);
}

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank = {};
    int nproc = {};
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nproc);

    auto pi = calculate_pi(rank);

    double sum = {};
    MPI_Allreduce(&pi, &sum, 1, MPI_DOUBLE, MPI_SUM, MPI_COMM_WORLD);

    MPI_Barrier(MPI_COMM_WORLD);

    if (rank == 0) {
        printf("pi: %f\n", sum / nproc);
    }

    MPI_Finalize();
}
