#include <print>
#include <sstream>
int main() {
    std::stringstream ss;
    ss << "a42a";
    int value;
    ss >> value;
    std::println("{}", value * 2);
}
