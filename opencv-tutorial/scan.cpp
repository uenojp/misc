#include <cassert>

#include <ctime>
#include <print>

#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>

int main(int argc, char** argv) {
    if (argc != 2) {
        std::println(stderr, "Usage: {} FILE", argv[0]);
        return 1;
    }

    const cv::Mat source = cv::imread(argv[1], cv::IMREAD_COLOR);
    if (source.empty()) {
        std::println(stderr, "Failed to read image: {}", argv[1]);
        return 1;
    }

    assert((void*)source.data == (void*)source.ptr(0));
    std::println("Mat.data: {:p}", (void*)source.data);
    std::println("row(0): {:p}, row(1): {:p}", (void*)source.ptr(0), (void*)source.ptr(1));
    std::println("channels: {}", source.channels());

    // channel
    {
        const int nchannels = 4;
        const cv::Mat m(4, 4, CV_8UC(nchannels), cv::Scalar(0, 127, 255));
        assert(m.channels() == nchannels);
    }
    {
        const int nchannels = 2;
        const cv::Mat m(4, 4, CV_8UC(nchannels), cv::Scalar(0, 127, 255));
        assert(m.channels() == nchannels);
    }

    // elemSize: https://docs.opencv.org/3.4/d3/d63/classcv_1_1Mat.html#aba7a5ef00b22b70668fba575005dfe55
    {
        const cv::Mat m(4, 4, CV_16UC4, cv::Scalar(0, 1, 2, 3));
        assert(m.elemSize() == 16/8 * 4);
        std::println("step.buf[0]: {}, step.buf[1]: {}", m.step.buf[0], m.step.buf[1]);
        std::println("{}", m.rows == 1 || m.step == m.cols*m.elemSize());
    }


    return 0;
    const size_t repetitions = 128;
    const int divide_with = 10;
    struct timespec start, end;

    clock_gettime(CLOCK_MONOTONIC, &start);

    clock_gettime(CLOCK_MONOTONIC, &end);
}
