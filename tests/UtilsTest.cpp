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

	TEST_CASE("is_close works on obvious values")
	{
		CHECK(utils::is_close(0, 0));
		CHECK(utils::is_close(1, 1));
		CHECK(!utils::is_close(0, 1));
	}

	TEST_CASE("is_close works for values with smaller differences")
	{
		CHECK(utils::is_close(5.0, 5.00000000001));
	}

	TEST_CASE("is_close rejects values with too little difference")
	{
		CHECK(!utils::is_close(5.0, 5.0000000001));
	}
}
