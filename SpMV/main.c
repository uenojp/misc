#include "mm2coo.c"
#include "coo2csr.c"

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s [Matrix Market File]\n", argv[0]);
        exit(0);
    }

    uint32_t n = 0;
    uint32_t nnz = 0;
    uint32_t* coo_row_index = NULL;
    uint32_t* coo_col_index = NULL;
    double* coo_value = NULL;

    if (mm2coo(argv[1], &coo_row_index, &coo_col_index, &coo_value, &n, &nnz) < 0) {
        fprintf(stderr, "failed to read %s as Matrix Market format\n", argv[1]);
        exit(1);
    }

    printf("n: %d, nnz: %d\n", n, nnz);
    print_coo(coo_row_index, coo_col_index, coo_value, nnz);
}
