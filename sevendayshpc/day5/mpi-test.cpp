#include <cstdio>
#include <vector>
#include <mpi.h>

const int L = 8;

struct Info {
    int rank;
    int nprocs;
    int GX, GY;
    int grid_x, grid_y;
    int grid_size_x, grid_size_y;
};

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
    const int start = info.rank * (info.grid_size_x * info.grid_size_y);

    int i = 0;
    for (int y = 1; y < info.grid_size_y + 1; y++) {
        for (int x = 1; x < info.grid_size_x + 1; x++) {
            grid[x + (info.grid_size_x + 2) * y] = start + i;
            i++;
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

void dump(std::vector<int> const &grid, Info const &info) {
    for (int i = 0; i < info.nprocs; i++) {
        MPI_Barrier(MPI_COMM_WORLD);
        if (i == info.rank) {
            dump_grid(grid, info);
        }
    }
}

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    Info info;
    setup_info(info);

    std::vector<int> grid((info.grid_size_x + 2) * (info.grid_size_y + 2), 0);

    init(grid, info);
    dump(grid, info);

    MPI_Finalize();
}

