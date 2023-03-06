#include "coo2csr.c"
#include "mm2coo.c"

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s [Matrix Market File]\n", argv[0]);
        exit(0);
    }

    //
    // read MatrixMarket Format
    //
    uint32_t n = 0;
    uint32_t nnz = 0;
    uint32_t* coo_row_index = NULL;
    uint32_t* coo_col_index = NULL;
    double* coo_value = NULL;

    if (mm2coo(argv[1], &coo_row_index, &coo_col_index, &coo_value, &n, &nnz) < 0) {
        fprintf(stderr, "failed to read %s as Matrix Market format\n", argv[1]);
        exit(1);
    }

    // printf("n: %d, nnz: %d\n", n, nnz);
    // print_coo(coo_row_index, coo_col_index, coo_value, nnz);

    // matrix product with coo
    double* x1 = (double*)malloc((size_t)n * sizeof(double));
    double* y1 = (double*)malloc((size_t)n * sizeof(double));

    for (uint32_t i = 0; i < n; i++) {
        x1[i] = (double)(i * i);
        y1[i] = 0.0;
    }
    for (uint32_t i = 0; i < nnz; i++) {
        y1[coo_row_index[i]] += coo_value[i] * x1[coo_col_index[i]];
    }
    for (uint32_t i = 0; i < n; i++) {
        printf("%lf%c", y1[i], i == n - 1 ? '\n' : ' ');
    }

    //
    // coo to csr
    //
    uint32_t* csr_row_ptr = NULL;
    uint32_t* csr_col_index = NULL;
    double* csr_value = NULL;

    // NOTE: coo_* must be sorted by row
    if (coo2csr(coo_row_index, coo_col_index, coo_value, &csr_row_ptr, &csr_col_index, &csr_value,
                n, nnz) < 0) {
        fprintf(stderr, "failed to convert from COO to CSR\n");
        exit(1);
    }

    // print_csr(csr_row_ptr, csr_col_index, csr_value, n);

    // matrix product with csr
    double* x2 = (double*)malloc((size_t)n * sizeof(double));
    double* y2 = (double*)malloc((size_t)n * sizeof(double));

    for (uint32_t i = 0; i < n; i++) {
        x2[i] = (double)(i * i);
        y2[i] = 0.0;
    }
    for (uint32_t i = 0; i < n; i++) {
        for (uint32_t j = csr_row_ptr[i]; j < csr_row_ptr[i + 1]; j++) {
            y2[i] += csr_value[j] * x2[csr_col_index[j]];
        }
    }
    for (uint32_t i = 0; i < n; i++) {
        printf("%lf%c", y2[i], i == n - 1 ? '\n' : ' ');
    }
}
