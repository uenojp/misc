// https://docs.opencv.org/4.x/d6/d6d/tutorial_mat_the_basic_image_container.html

#include <vector>
#include <iostream>
#include <print>
#include <cassert>

#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>

int main(int argc, char** argv) {
    if (argc != 2) {
        std::println(stderr, "Usage: {} FILE", argv[0]);
        return 1;
    }

    const cv::Mat A = cv::imread(argv[1], cv::IMREAD_COLOR);

    {
        // Image data is shared between these matrices
        const cv::Mat B(A);   // image data is not allocated
        const cv::Mat C = A;  // image data is not allocated
        assert(A.data == B.data);
        assert(A.data == C.data);

        // Also be shared
        const cv::Mat D(A, cv::Rect(10, 10, 100, 100));
        assert(D.data != A.data);
        const cv::Mat E1 = A(/* row */ cv::Range::all(), /* col */ cv::Range(1, 100));
        assert(E1.data != A.data);
        const cv::Mat E2 = A(/* row */ cv::Range::all(), /* col */ cv::Range(0, 100));
        assert(E2.data == A.data);

        // Image data is newly allocated
        const cv::Mat F = A.clone();
        assert(F.data != A.data);
        cv::Mat G;
        A.copyTo(G);
        assert(G.data != A.data);
    }

    {
        cv::Mat M(4, 4, CV_8UC3, cv::Scalar(0, 127, 255));
        cv::print(M);
        std::println("");

        // 2x2x2
        const int dims = 3;
        const int sz[dims] = {2, 2, 2};
        const cv::Mat L(dims, sz, CV_8UC(1), cv::Scalar::all(0));
        assert(L.dims == dims);

        // uninitialized
        M.create(4, 4, CV_8UC(2));
        cv::print(M);
        std::println("");

        // identity matrix
        const cv::Mat E = cv::Mat::eye(4, 4, CV_64F);
        cv::print(E);
        std::println("");
        // one matrix
        const cv::Mat O = cv::Mat::ones(4, 4, CV_64F);
        cv::print(O);
        std::println("");
        // zero matrix
        const cv::Mat Z = cv::Mat::zeros(4, 4, CV_32F);
        cv::print(Z);
        std::println("");

        // initialize
        const cv::Mat C = (cv::Mat_<double>(3, 3) << 0, -1, 2, -3, 4, -5, 6, -7, 8);
        cv::print(C);
        std::println("");

        // extrat row 1
        const cv::Mat R = C.row(1).clone();
        cv::print(R);
        std::println("");
    }

    {
        const cv::Point2f P2(5, 1);
        std::cout << P2 << std::endl;
        const cv::Point3d P3(3, 1, 4);
        std::cout << P3 << std::endl;
        std::vector<double> v{3, 1, 4, 1, 5, 9, 2, 6, 5};
        const cv::Mat M(v);
        cv::print(M.reshape(3));
        std::println("");
    }

    return 0;
}
