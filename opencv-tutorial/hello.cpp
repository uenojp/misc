// https://docs.opencv.org/4.x/db/deb/tutorial_display_image.html

#include <opencv2/core/utility.hpp>
#include <print>

#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>

int main(int argc, char** argv) {
    if (argc != 2) {
        std::println(stderr, "Usage: {} FILE", argv[0]);
        return 1;
    }

    const cv::Mat image = cv::imread(argv[1], cv::IMREAD_COLOR);
    if (image.empty()) {
        std::println(stderr, "could not read the image {}", argv[1]);
        return 1;
    }

    cv::imshow("Window", image);
    int key = cv::waitKey(0);
    if (key == 's') {
        cv::imwrite("output.jpg", image);
    }

    return 0;
}
