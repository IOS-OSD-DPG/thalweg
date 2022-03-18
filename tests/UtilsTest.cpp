#include "Utils.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("UtilsTest")
{

TEST_CASE("split returns single string when char not present")
{
	CHECK(utils::split("no newlines", '\n') == std::vector<std::string>{"no newlines"});
}

}
