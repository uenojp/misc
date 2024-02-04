#include <mpi.h>
#include <stddef.h>
#include <stdint.h>
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

    // PingPong each other with message sizes from 1B to 1GiB.
    const size_t from = 1L;
    const size_t to = 1L * 1024L * 1024L * 1024L;

    for (size_t message_size_byte = from; message_size_byte <= to; message_size_byte <<= 1) {
        // Prepare send/recv data.
        uint8_t *message = NULL;

        if ((message = (uint8_t *)malloc(sizeof(uint8_t) * message_size_byte)) == NULL) {
            perror("PingPong: malloc");
            MPI_Abort(MPI_COMM_WORLD, EXIT_FAILURE);
        }
        for (size_t i = 0; i < message_size_byte; i++) {
            message[i] = (uint8_t)i;
        }

        // Let's go PingPong.
        MPI_Status status;

        const double start = MPI_Wtime();
        if (rank & 1) {
            MPI_Send(message, message_size_byte, MPI_BYTE, 0, 0, MPI_COMM_WORLD);
            MPI_Recv(message, message_size_byte, MPI_BYTE, 0, 1, MPI_COMM_WORLD, &status);
        } else {
            MPI_Recv(message, message_size_byte, MPI_BYTE, 1, 0, MPI_COMM_WORLD, &status);
            MPI_Send(message, message_size_byte, MPI_BYTE, 1, 1, MPI_COMM_WORLD);
        }
        const double end = MPI_Wtime();

        MPI_Barrier(MPI_COMM_WORLD);

        if (rank == 0) {
            printf("%ld Byte %lf MiB/s\n", message_size_byte, (double)message_size_byte / ((end - start) / 2.0) / (1024.0 * 1024.0));
        }

        free(message);
    }

    MPI_Finalize();
}
