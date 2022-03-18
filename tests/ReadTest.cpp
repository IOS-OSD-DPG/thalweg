#include "Read.hpp"

#include "doctest.h"

#include <sstream>

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

TEST_SUITE("ReadTest")
{

TEST_CASE("read_data can convert a single line")
{
	std::istringstream contents("49-12-00.000N 112-56-24.360W 100.000\n");
	std::vector<Location> expected = {Location{Coordinate{49.2, -112.94}, 100.000}};
	CHECK(read_data(contents) == expected);
}

}
