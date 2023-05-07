#include <mpi.h>

#include <cassert>
#include <cstdio>
#include <vector>

const int L = 8;

struct Info {
    int rank;
    int nprocs;
    int GX, GY;
    int grid_x, grid_y;
    int grid_size_x, grid_size_y;
};

void print_info(Info const &info) {
    if (info.rank != 0)
        return;

    printf(
        "rank: %d\n"
        "nproc: %d\n"
        "GX: %d\n"
        "GY: %d\n"
        "grid_x: %d\n"
        "grid_y: %d\n"
        "grid_size_x: %d\n"
        "grid_size_y: %d\n",
        info.rank, info.nprocs, info.GX, info.GY, info.grid_x, info.grid_y, info.grid_size_x,
        info.grid_size_y);
}

void setup_info(Info &info) {
    int rank;
    int nprocs;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nprocs);
    info.rank = rank;
    info.nprocs = nprocs;

    int d[2] = {};
    MPI_Dims_create(nprocs, 2, d);
    info.GX = d[0];
    info.GY = d[1];

    info.grid_x = rank % info.GX;
    info.grid_y = rank / info.GX;
    info.grid_size_x = L / info.GX;
    info.grid_size_y = L / info.GY;
}

void init(std::vector<int> &grid, Info const &info) {
    int i = info.rank * (info.grid_size_x * info.grid_size_y);

    for (int y = 1; y < info.grid_size_y + 1; y++) {
        for (int x = 1; x < info.grid_size_x + 1; x++) {
            grid[x + (info.grid_size_x + 2) * y] = i;
            i++;
        }
    }
}

void gather(std::vector<int> const &grid, std::vector<int> &global, Info const &info) {
    assert(global.size() == (info.GX * info.grid_size_x * info.GY * info.grid_size_y));  // == L * L

    std::vector<int> local;
    local.reserve(info.grid_size_x * info.grid_size_y);
    for (int y = 1; y < info.grid_size_y + 1; y++) {
        for (int x = 1; x < info.grid_size_x + 1; x++) {
            local.push_back(grid[x + (info.grid_size_x + 2) * y]);
        }
    }

    MPI_Gather(local.data(), local.size(), MPI_INT, global.data(), local.size(), MPI_INT, 0,
               MPI_COMM_WORLD);
}

void reordering(std::vector<int> &global, Info const &info) {
    std::vector<int> tmp(global.size());
    std::copy(global.begin(), global.end(), tmp.begin());

    const int lx = info.grid_size_x;
    const int ly = info.grid_size_y;
    int i = 0;
    for (int r = 0; r < info.nprocs; r++) {
        int rx = r % info.GX;
        int ry = r / info.GX;
        int sx = rx * lx;
        int sy = ry * ly;
        for (int iy = 0; iy < ly; iy++) {
            for (int ix = 0; ix < lx; ix++) {
                int index = (sx + ix) + (sy + iy) * L;
                global[index] = tmp[i];
                i++;
            }
        }
    }
}

void dump_grid(std::vector<int> const &grid, Info const &info) {
    printf("rank: %d\n", info.rank);
    for (int y = 0; y < info.grid_size_y + 2; y++) {
        for (int x = 0; x < info.grid_size_x + 2; x++) {
            printf(" %03d", grid[x + (info.grid_size_x + 2) * y]);
        }
        printf("\n");
    }
}

void dump_local(std::vector<int> const &grid, Info const &info) {
    for (int i = 0; i < info.nprocs; i++) {
        MPI_Barrier(MPI_COMM_WORLD);
        if (i == info.rank) {
            dump_grid(grid, info);
        }
    }
}

void dump_global(std::vector<int> const &global, Info const &info) {
    if (info.rank != 0)
        return;

    printf("global\n");
    for (int y = 0; y < L; y++) {
        for (int x = 0; x < L; x++) {
            printf(" %03d", global[x + y * L]);
        }
        printf("\n");
    }
}

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    // setup MPI info such as rank, nprocs and grid size ...
    Info info;
    setup_info(info);
    print_info(info);

    // setup grid
    std::vector<int> local((info.grid_size_x + 2) * (info.grid_size_y + 2), 0);
    init(local, info);
    dump_local(local, info);

    // gather grid to global data
    std::vector<int> global(L * L, 0);
    gather(local, global, info);
    dump_global(global, info);

    // reporder global data
    reordering(global, info);
    dump_global(global, info);

    MPI_Finalize();
}
