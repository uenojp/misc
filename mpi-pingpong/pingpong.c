#include <mpi.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    int rank;
    int nprocesses;

    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &nprocesses);

    if (nprocesses != 2) {
        fprintf(stderr, "PingPong: Specify just two processes\n");
        MPI_Abort(MPI_COMM_WORLD, EXIT_FAILURE);
    }

    // PingPong each other with message sizes from 1B to 1GB.
    const size_t from = 1;
    const size_t to = 1024 * 1024 * 1024;

    for (size_t message_size = from; message_size < to; message_size <<= 1) {
        // Prepare send/recv data.
        double *message = NULL;

        if ((message = (double *)malloc(sizeof(double) * message_size)) == NULL) {
            perror("PingPong: malloc");
            MPI_Abort(MPI_COMM_WORLD, EXIT_FAILURE);
        }
        for (size_t i = 0; i < message_size; i++) {
            message[i] = (double)i;
        }

        // Let's go PingPong.
        MPI_Status status;

        const double start = MPI_Wtime();
        if (rank == 0) {
            MPI_Send(message, message_size, MPI_DOUBLE, 1, 0, MPI_COMM_WORLD);
            MPI_Recv(message, message_size, MPI_DOUBLE, 1, 1, MPI_COMM_WORLD, &status);
        } else {
            MPI_Recv(message, message_size, MPI_DOUBLE, 0, 0, MPI_COMM_WORLD, &status);
            MPI_Send(message, message_size, MPI_DOUBLE, 0, 1, MPI_COMM_WORLD);
        }
        const double end = MPI_Wtime();

        if (rank == 0) {
            printf("%lf GB/s\n", message_size / (end - start) / 2 / (1024 * 1024));
        }

        free(message);
    }

    MPI_Finalize();
}
