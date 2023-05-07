#include <cassert>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

const int32_t L = 128;

const size_t TOTAL_STEP = 20000;
const size_t INTERVAL = 200;

const double F = 0.04;
const double k = 0.06075;
const double Du = 0.05;
const double Dv = 0.1;
const double dt = 0.2;

double laplacian(int32_t ix, int32_t iy, std::vector<double> const &v) {
    assert(ix >= 0 && iy >= 0);

    double r = 0.0;
    r += v[(ix - 1) + (iy)*L];
    r += v[(ix + 1) + (iy)*L];
    r += v[(ix) + (iy - 1) * L];
    r += v[(ix) + (iy + 1) * L];
    r -= 4 * v[(ix) + (iy)*L];

    return r;
}

void step(std::vector<double> const &u, std::vector<double> const &v, std::vector<double> &u2,
          std::vector<double> &v2) {
    for (int32_t ix = 1; ix < L - 1; ix++) {
        for (int32_t iy = 1; iy < L - 1; iy++) {
            const int32_t i = ix + iy * L;

            const double dudt = Du * laplacian(ix, iy, u) + u[i] * u[i] * v[i] - (F + k) * u[i];
            const double dvdt = Dv * laplacian(ix, iy, v) - u[i] * u[i] * v[i] + F * (1.0 - v[i]);

            u2[i] = u[i] + dt * dudt;
            v2[i] = v[i] + dt * dvdt;
        }
    }
}

void init(std::vector<double> &u, std::vector<double> &v) {
    int32_t around = 3;
    int32_t from = L / 2 - around;
    int32_t to = L / 2 + around;

    assert(from >= 0 && to >= 0);

    for (int32_t j = from; j < to; j++) {
        for (int32_t i = from; i < to; i++) {
            u[i + j * L] = 0.7;
        }
    }

    around = 6;
    from = L / 2 - around;
    to = L / 2 + around;

    assert(from >= 0 && to >= 0);

    for (int32_t j = from; j < to; j++) {
        for (int32_t i = from; i < to; i++) {
            v[i + j * L] = 0.9;
        }
    }
}

void save_as_dat(std::vector<double> data) {
    static int index = 0;
    char filename[256];
    sprintf(filename, "data/conf%03d.dat", index);
    std::cout << filename << std::endl;
    std::ofstream ofs(filename, std::ios::binary);
    ofs.write((char *)(data.data()), sizeof(double) * L * L);
    index++;
}

int main(void) {
    const size_t size = static_cast<size_t>(L) * static_cast<size_t>(L);
    std::vector<double> u(size, 0.0), v(size, 0.0);
    std::vector<double> u2(size, 0.0), v2(size, 0.0);

    init(u, v);

    for (size_t i = 0; i < TOTAL_STEP; i++) {
        if (i & 1) {
            step(u2, v2, u, v);
        } else {
            step(u, v, u2, v2);
        }

        if (i % INTERVAL == 0) {
            save_as_dat(u);
        }
    }
}

