#include <mpi.h>

#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

const std::size_t L = 128;
const std::size_t STEP = 10000;
const std::size_t DUMP = 1000;

void onestep(std::vector<double> &lattice, double h, int rank, int nprocess) {
    assert(L % nprocess == 0);

    const std::size_t size = lattice.size();
    std::vector<double> pre_lattice(size);

    std::copy(lattice.begin(), lattice.end(), pre_lattice.begin());

    const int left_rank = (rank - 1 + nprocess) % nprocess;
    const int right_rank = (rank + 1) % nprocess;

    MPI_Status _status;
    // send right side to right_rank, recieve left side from left_rank
    MPI_Sendrecv(&lattice[size - 2], 1, MPI_DOUBLE, right_rank, 0, &pre_lattice[0], 1, MPI_DOUBLE,
                 left_rank, 0, MPI_COMM_WORLD, &_status);
    // send left side to left_rank, recieve right side from right_rank
    MPI_Sendrecv(&lattice[1], 1, MPI_DOUBLE, left_rank, 0, &pre_lattice[size - 1], 1, MPI_DOUBLE,
                 right_rank, 0, MPI_COMM_WORLD, &_status);

    for (std::size_t i = 1; i < size - 1; i++) {
        lattice[i] += h * (pre_lattice[i + 1] - 2 * pre_lattice[i] + pre_lattice[i - 1]);
    }
}

void dump(std::vector<double> &data) {
    static int index = 0;
    char filename[256];

    sprintf(filename, "mpi-thermal/data%03d.dat", index);
    std::cout << filename << std::endl;

    std::ofstream ofs(filename);

    for (std::size_t i = 0; i < data.size(); i++) {
        ofs << i << " " << data[i] << std::endl;
    }
    index += 1;
}

void mpi_dump(std::vector<double> &local, int rank, int nprocess) {
    std::vector<double> global(L);

    assert(L % nprocess == 0);

    const int size = L / nprocess;
    MPI_Gather(&local[1], size, MPI_DOUBLE, global.data(), size, MPI_DOUBLE, 0, MPI_COMM_WORLD);
    if (rank == 0) {
        dump(global);
    }
}

void fixed_temperature(std::vector<double> &lattice, int rank, int nprocess) {
    const double h = 0.01;
    const double q = 1.0;
    const std::size_t s = L / nprocess;

    for (std::size_t i = 0; i < STEP; i++) {
        onestep(lattice, h, rank, nprocess);

        if (rank == (L / 4 / s)) {
            lattice[L / 4 - rank * s + 1] = q;
        }
        if (rank == (3 * L / 4 / s)) {
            lattice[3 * L / 4 - rank * s + 1] = -q;
        }
        if ((i % DUMP) == 0)
            mpi_dump(lattice, rank, nprocess);
    }
}

void uniform_heating(std::vector<double> &lattice, int rank, int nprocess) {
    const double h = 0.2;
    const double q = 1.0;

    for (std::size_t i = 0; i < STEP; i++) {
        onestep(lattice, h, rank, nprocess);
        for (auto &s : lattice) {
            s += q * h;
        }
        if (rank == 0) {
            lattice[1] = 0.0;
        }
        if (rank == nprocess - 1) {
            lattice[lattice.size() - 2] = 0.0;
        }
        if ((i % DUMP) == 0)
            mpi_dump(lattice, rank, nprocess);
    }
}

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    int rank;
    int nprocess;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nprocess);

    std::size_t size = L / nprocess;
    std::vector<double> local(size + 2);
    fixed_temperature(local, rank, nprocess);
    // uniform_heating(local, rank, nprocess);

    MPI_Finalize();
}
