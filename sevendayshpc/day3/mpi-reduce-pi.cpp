#include <mpi.h>

#include <cstdio>
#include <cstddef>
#include <random>

const size_t n = 1024 * 1024;

double calculate_pi(const int seed) {
    std::mt19937 mt(seed);
    std::uniform_real_distribution<double> ud(0.0, 1.0);

    size_t m = 0;

    for (size_t i = 0; i < n; i++) {
        auto x = ud(mt);
        auto y = ud(mt);
        if (x * x + y * y <= 1.0) {
            m += 1;
        }
    }

    return 4.0 * m / n;
}

int main(int argc, char** argv) {
    MPI_Init(&argc, &argv);

    int rank = -1;
    int nproc = -1;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nproc);

    const auto pi = calculate_pi(rank);
    const auto pi2 = pi * pi;
    double sum_pi = 0.0;
    double sum_pi2 = 0.0;

    MPI_Reduce(&pi, &sum_pi, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);
    MPI_Reduce(&pi2, &sum_pi2, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

    MPI_Barrier(MPI_COMM_WORLD);

    if (rank == 0) {
        const auto ave_pi = sum_pi / nproc;
        const auto var_pi = sum_pi2 / (nproc - 1) - ave_pi * ave_pi * nproc / (nproc - 1);
        const auto dev_pi = sqrt(var_pi);

        printf("%lf +- %lf\n", ave_pi, dev_pi);
    }

    MPI_Finalize();
}

