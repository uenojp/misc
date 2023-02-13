#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

const std::size_t L = 128;
const std::size_t STEP = 10000;
const std::size_t DUMP = 1000;

void onestep(std::vector<double> &lattice, double h) {
    std::vector<double> pre_lattice(L);
    std::copy(lattice.begin(), lattice.end(), pre_lattice.begin());

    for (std::size_t i = 1; i < L - 1; i++) {
        lattice[i] += h * (pre_lattice[i + 1] - 2 * pre_lattice[i] + pre_lattice[i - 1]);
    }
    lattice[0] += h * (pre_lattice[L - 1] - 2 * pre_lattice[0] + pre_lattice[1]);
    lattice[L - 1] += h * (pre_lattice[L - 2] - 2 * pre_lattice[L - 1] + pre_lattice[0]);
}

void dump(std::vector<double> &data) {
    static int index = 0;
    char filename[256];

    sprintf(filename, "data%03d.dat", index);
    std::cout << filename << std::endl;

    std::ofstream ofs(filename);

    for (std::size_t i = 0; i < data.size(); i++) {
        ofs << i << " " << data[i] << std::endl;
    }
    index += 1;
}

void fixed_temperature(std::vector<double> &lattice) {
    const double h = 0.01;
    const double q = 1.0;

    for (std::size_t i = 0; i < STEP; i++) {
        onestep(lattice, h);
        lattice[L / 4] = q;
        lattice[3 * L / 4] = -q;
        if ((i % DUMP) == 0)
            dump(lattice);
    }
}

void uniform_heating(std::vector<double> &lattice) {
    const double h = 0.2;
    const double q = 1.0;

    for (std::size_t i = 0; i < STEP; i++) {
        onestep(lattice, h);
        for (auto &s : lattice) {
            s += q * h;
        }
        lattice[0] = 0.0;
        lattice[L - 1] = 0.0;
        if ((i % DUMP) == 0)
            dump(lattice);
    }
}

int main(int argc, char **argv) {
    std::vector<double> lattice(L, 0.0);
    fixed_temperature(lattice);
    // uniform_heating(lattice);
}
