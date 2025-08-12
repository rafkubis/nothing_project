#include <gtest/gtest.h>

extern std::vector<size_t> kernels(const std::string& str);

TEST(kernels_test, basic_test) {
    std::string str = "test";
    auto result = kernels(str);

    EXPECT_FLOAT_EQ(result[0], 3);
}

