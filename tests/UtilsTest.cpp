#include "Utils.hpp"

#include "DisplayVector.hpp"

#include "doctest.h"

#include <iostream>

using namespace thalweg;

TEST_SUITE("UtilsTest")
{

	TEST_CASE("split returns single string when char not present")
	{
		CHECK(utils::split("no newlines", '\n') == std::vector<std::string> {"no newlines"});
	}

	TEST_CASE("split returns two values when char appears once")
	{
		CHECK(utils::split("easy-mode", '-') == std::vector<std::string> {"easy", "mode"});
		CHECK(utils::split("-hardmode", '-') == std::vector<std::string> {"", "hardmode"});
		CHECK(utils::split("hardmode-", '-') == std::vector<std::string> {"hardmode", ""});
	}
}
