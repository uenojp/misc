#pragma once

// Matrix Market Format
// https://math.nist.gov/MatrixMarket/formats.html

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#define MM_HEADER_LINE "%%MatrixMarket matrix coordinate real general"

int read_mm_header(FILE* fp, uint32_t* nptr, uint32_t* nnzptr) {
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

    // read rows, columns and entries
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

    *nptr = n1;
    *nnzptr = entries;

    return 0;
}

int read_mm_data(FILE* fp, uint32_t** coo_row_index, uint32_t** coo_col_index, double** coo_value,
                 uint32_t nnz) {
    char* line = NULL;
    size_t len = 0;
    ssize_t nread = 0;
    uint32_t index = 0;

    while ((nread = getline(&line, &len, fp)) != EOF && index < nnz) {
        line[nread - 1] = '\0';

        char* endptr = NULL;

        const uint32_t i = (uint32_t)strtol(line, &endptr, 10);
        if (*endptr != ' ') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        const uint32_t j = (uint32_t)strtol(endptr + 1, &endptr, 10);
        if (*endptr != ' ') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        const double value = (double)strtod(endptr + 1, &endptr);
        if (*endptr != '\0') {
            fprintf(stderr, "invalid numeric data format: '%s'\n", line);
            return -1;
        }

        (*coo_row_index)[index] = i;
        (*coo_col_index)[index] = j;
        (*coo_value)[index] = value;

        index++;
    }

    // less or more data
    if (index != nnz || !feof(fp)) {
        fprintf(stderr, "mismatch between nnz and number of data\n");
        return -1;
    }
}

int mm2coo(const char* pathname, uint32_t** coo_row_index, uint32_t** coo_col_index,
           double** coo_value, uint32_t* nptr, uint32_t* nnzptr) {
    FILE* fp = NULL;

    if ((fp = fopen(pathname, "r")) == NULL) {
        perror("fopen");
        return -1;
    }

    if (read_mm_header(fp, nptr, nnzptr) < 0) {
        fprintf(stderr, "invalid Matrix Market header\n");
        fclose(fp);
        return -1;
    }

    uint32_t n = *nptr;
    uint32_t nnz = *nnzptr;

    *coo_row_index = (uint32_t*)malloc((size_t)nnz * sizeof(uint32_t));
    *coo_col_index = (uint32_t*)malloc((size_t)nnz * sizeof(uint32_t));
    *coo_value = (double*)malloc((size_t)nnz * sizeof(double));
    if (!(*coo_row_index) || !(*coo_col_index) || !(*coo_value)) {
        return -1;
    }

    if (read_mm_data(fp, coo_row_index, coo_col_index, coo_value, nnz) < 0) {
        fprintf(stderr, "invalid Matrix Market data\n");
        fclose(fp);
        return -1;
    }

    fclose(fp);

    return 0;
}

void print_coo(const uint32_t* coo_row_index, const uint32_t* coo_col_index, double* coo_value,
               uint32_t nnz) {
    for (uint32_t i = 0; i < nnz; i++) {
        printf("%d %d %lf\n", coo_row_index[i], coo_col_index[i], coo_value[i]);
    }
}
