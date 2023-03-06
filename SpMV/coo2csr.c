#pragma once

#include <stdbool.h>

#include "mm2coo.c"

bool is_sorted_by_row(const uint32_t* coo_row_index, const uint32_t* coo_col_index, uint32_t nnz,
                      uint32_t n) {
    for (int32_t i = 0; i < nnz - 1; i++) {
        if ((coo_row_index[i] * n + coo_col_index[i]) >
            (coo_row_index[i + 1] * n + coo_col_index[i + 1])) {
            return false;
        }
    }

    return true;
}

int coo2csr(const uint32_t* coo_row_index, const uint32_t* coo_col_index, const double* coo_value,
            uint32_t** csr_row_ptr_ptr, uint32_t** csr_col_index_ptr, double** csr_value_ptr,
            uint32_t n, uint32_t nnz) {
    if (!is_sorted_by_row(coo_row_index, coo_col_index, nnz, n)) {
        fprintf(stderr, "COO must be sorted by row to convert from COO to CSR\n");
        return -1;
    }

    *csr_row_ptr_ptr = (uint32_t*)malloc((size_t)(n + 1) * sizeof(uint32_t));
    *csr_col_index_ptr = (uint32_t*)malloc((size_t)nnz * sizeof(uint32_t));
    *csr_value_ptr = (double*)malloc((size_t)nnz * sizeof(double));
    if (!(*csr_row_ptr_ptr) || !(*csr_col_index_ptr) || !(*csr_value_ptr)) {
        return -1;
    }

    for (uint32_t i = 0; i < n + 1; i++) {
        (*csr_row_ptr_ptr)[i] = 0;
    }

    for (uint32_t i = 0; i < nnz; i++) {
        // coo_row_index is 0-based indexing
        (*csr_row_ptr_ptr)[coo_row_index[i] + 1]++;
        (*csr_col_index_ptr)[i] = coo_col_index[i];
        (*csr_value_ptr)[i] = coo_value[i];
    }

    for (uint32_t i = 0; i < n; i++) {
        (*csr_row_ptr_ptr)[i + 1] += (*csr_row_ptr_ptr)[i];
    }

    return 0;
}

void print_csr(const uint32_t* csr_row_ptr, const uint32_t* csr_col_index, const double* csr_value,
               uint32_t n) {
    // O(nnz)
    // nnz = Sum of [row_ptr[i], row_ptr[i + 1])
    for (uint32_t i = 0; i < n; i++) {
        for (uint32_t j = csr_row_ptr[i]; j < csr_row_ptr[i + 1]; j++) {
            printf("%d %d %.13e\n", i, csr_col_index[j], csr_value[j]);
        }
    }
}
