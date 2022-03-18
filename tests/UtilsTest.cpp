#include "Utils.hpp"

#include "doctest.h"

#include <iostream>

using namespace thalweg;

// the standard forbids this, but it seems to work
namespace std
{
template<typename T>
auto operator<<(std::ostream& os, std::vector<T> const& vec) -> std::ostream&
{
	os << "[";
	for (size_t i = 0; i < vec.size(); ++i)
	{
		if (i < vec.size() - 1)
			os << vec[i] << ", ";
		else
			os << vec[i];
	}
	os << "]";
	return os;
}
}

TEST_SUITE("UtilsTest")
{

TEST_CASE("split returns single string when char not present")
{
	CHECK(utils::split("no newlines", '\n') == std::vector<std::string>{"no newlines"});
}

TEST_CASE("split returns two values when char appears once")
{
	CHECK(utils::split("easy-mode", '-') == std::vector<std::string>{"easy", "mode"});
	CHECK(utils::split("-hardmode", '-') == std::vector<std::string>{"", "hardmode"});
	CHECK(utils::split("hardmode-", '-') == std::vector<std::string>{"hardmode", ""});
}

}
