#include <cstdio>
#include <random>

const size_t n = 1024 * 1024;

double calculate_pi(const int seed) {
    std::mt19937 mt(seed);
    std::uniform_real_distribution<double> ud(0.0, 1.0);

    std::size_t m = 0;

    for (std::size_t i = 0; i < n; i++) {
        auto x = ud(mt);
        auto y = ud(mt);
        if (x * x + y * y <= 1.0) {
            m += 1;
        }
    }
    
    return 4.0 * static_cast<double>(m) / static_cast<double>(n);
}

int main(int argc, char** argv) {
    auto pi = calculate_pi(0);
    printf("%f\n", pi);
}

