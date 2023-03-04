#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#define MM_HEADER_LINE "%%MatrixMarket matrix coordinate real general"

int read_mm_header(FILE* fp, uint32_t* n, uint32_t* nnz) {
    char* line = NULL;
    size_t len = 0;
    ssize_t nread = 0;

    // read header line
    // e.g. '%%MatrixMarket matrix coordinate real general'
    const size_t magic_len = sizeof(MM_HEADER_LINE) - 1;

    if ((nread = getline(&line, &len, fp)) < 0) {
        return -1;
    }

    if ((size_t)nread < magic_len || strncmp(line, MM_HEADER_LINE, magic_len)) {
        fprintf(stderr, "support only %s\n", MM_HEADER_LINE);
        return -1;
    }

    // skip comments
    while ((nread = getline(&line, &len, fp))) {
        if (nread < 0) {
            return -1;
        }

        if (line[0] != '%') {
            break;
        }
    }
    line[nread - 1] = '\0';

    // read rows, columns, entries
    // format: '<rows> <columns> <entries>'
    char* endptr;
    uint32_t n1 = 0;
    uint32_t n2 = 0;
    uint32_t entries = 0;

    n1 = (uint32_t)strtol(line, &endptr, 10);
    if (*endptr != ' ') {
        fprintf(stderr, "invalid matrix entries format: '%s'\n", line);
        return -1;
    }

    n2 = (uint32_t)strtol(endptr + 1, &endptr, 10);
    if (*endptr != ' ') {
        fprintf(stderr, "invalid matrix entries format: '%s'\n", line);
        return -1;
    }

    if (n1 != n2) {
        fprintf(stderr, "support only square matrix\n");
        return -1;
    }

    entries = (uint32_t)strtol(endptr + 1, &endptr, 10);
    if (*endptr != '\0') {
        fprintf(stderr, "invalid matrix entries format: '%s'\n", line);
        return -1;
    }

    *n = n1;
    *nnz = entries;

    return 0;
}

int mm2coo(FILE* fp, uint32_t* coo_row_index, uint32_t* coo_col_index, double* coo_val,
           uint32_t nnz) {
    char* line = NULL;
    size_t len = 0;
    ssize_t nread = 0;
    uint32_t index = 0;

    while ((nread = getline(&line, &len, fp)) != EOF && index < nnz) {
        line[nread - 1] = '\0';

        char* endptr;
        uint32_t i = 0;
        uint32_t j = 0;
        double value = 0;

        i = (uint32_t)strtol(line, &endptr, 10);
        if (*endptr != ' ') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        j = (uint32_t)strtol(endptr + 1, &endptr, 10);
        if (*endptr != ' ') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        value = (double)strtod(endptr + 1, &endptr);
        if (*endptr != '\0') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        coo_row_index[index] = i;
        coo_col_index[index] = j;
        coo_val[index] = value;

        index++;
    }

    // less or more data
    if (index != nnz || !feof(fp)) {
        fprintf(stderr, "mismatch between nnz and number of data\n");
        return -1;
    }

    return 0;
}

void print_coo(uint32_t* coo_row_index, uint32_t* coo_col_index, double* coo_val, uint32_t nnz) {
    for (uint32_t i = 0; i < nnz; i++) {
        printf("%d %d %lf\n", coo_row_index[i], coo_col_index[i], coo_val[i]);
    }
}

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s [Matrix Market File]\n", argv[0]);
        exit(0);
    }

    FILE* fp = NULL;
    uint32_t n = 0;
    uint32_t nnz = 0;

    if ((fp = fopen(argv[1], "r")) == NULL) {
        perror("fopen");
        exit(1);
    }

    if (read_mm_header(fp, &n, &nnz) < 0) {
        fprintf(stderr, "fialed to read Matrix Market header\n");
        exit(1);
    }

    printf("n: %d, nnz: %d\n", n, nnz);

    uint32_t* coo_row_index = (uint32_t*)malloc((size_t)nnz * sizeof(uint32_t));
    uint32_t* coo_col_index = (uint32_t*)malloc((size_t)nnz * sizeof(uint32_t));
    double* coo_val = (double*)malloc((size_t)nnz * sizeof(double));

    mm2coo(fp, coo_row_index, coo_col_index, coo_val, nnz);

    print_coo(coo_row_index, coo_col_index, coo_val, nnz);

    fclose(fp);
}
