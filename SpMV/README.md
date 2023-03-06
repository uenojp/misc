Inspired by https://zenn.dev/hishinuma_t/books/sparse-matrix-and-vector-product

Downloads sample data(Matrix Market Format)
```bash
curl -L -o- https://math.nist.gov/pub/MatrixMarket2/Harwell-Boeing/econaus/orani678.mtx.gz | zcat > test.mtx

# sort to create CSR format
(head -2 test.mtx; tail -n +3 test.mtx | sort -k1,1n -k2,2n) > sorted.test.mtx
```

