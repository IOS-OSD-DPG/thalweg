#include "Read.hpp"

#include "doctest.h"

#include <sstream>

using namespace thalweg;

namespace
{
	auto check(Coordinate const& lhs, Coordinate const& rhs) -> void
	{
		CHECK(lhs.latitude == doctest::Approx(rhs.latitude));
		CHECK(lhs.longitude == doctest::Approx(rhs.longitude));
	}

	auto check(Location const& lhs, Location const& rhs) -> void
	{
		check(lhs.coord, rhs.coord);
		CHECK(lhs.depth == doctest::Approx(rhs.depth));
	}

	template<typename T>
	auto check(std::vector<T> const& lhs, std::vector<T> const& rhs) -> void
	{
		CHECK(lhs.size() == rhs.size());
		size_t const length = std::min(lhs.size(), rhs.size());
		for (size_t i = 0; i < length; ++i)
		{
			check(lhs[i], rhs[i]);
		}
	}
}

TEST_SUITE("ReadTest")
{

	TEST_CASE("read_data can convert a single line")
	{
		std::istringstream contents("49-12-00.000N 112-56-24.360W 100.000\n");
		auto const expected = std::vector<Location> {
			Location {Coordinate {49.2, -112.94}, 100.000},
		};
		check(read_data(contents), expected);
	}

	TEST_CASE("read_data can convert multiple lines")
	{
		std::istringstream contents(
			"49-12-00.000N 112-56-24.360W 100.000\n49-12-00.000S 112-56-24.360E 100.000\n");
		auto const expected = std::vector<Location> {
			Location {Coordinate {49.2, -112.94}, 100.000},
			Location {Coordinate {-49.2, 112.94}, 100.000},
		};
		check(read_data(contents), expected);
	}

	TEST_CASE("read_data ignores lines that are obviously wrong")
	{
		std::istringstream contents(
			"\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n49-12-00.000N 112-56-24.360W 100.000\n");
		auto const expected = std::vector<Location> {
			Location {Coordinate {49.2, -112.94}, 100.000},
		};
		check(read_data(contents), expected);
	}

	TEST_CASE("read_corners can convert a single line")
	{
		std::istringstream contents("49-12-00.000N 112-56-24.360W\n");
		auto const expected = std::vector<Coordinate> {
			Coordinate {49.2, -112.94},
		};
		check(read_corners(contents), expected);
	}

	TEST_CASE("read_corners can convert multiple lines")
	{
		std::istringstream contents("49-12-00.000N 112-56-24.360W\n49-12-00.000S 112-56-24.360E\n");
		auto const expected = std::vector<Coordinate> {
			Coordinate { 49.2, -112.94},
			Coordinate {-49.2,  112.94},
		};
		check(read_corners(contents), expected);
	}

	TEST_CASE("read_corners ignores lines that are obviously wrong")
	{
		std::istringstream contents("\"Lat (DMS)\" \"Long (DMS)\"\n49-12-00.000N 112-56-24.360W\n");
		auto const expected = std::vector<Coordinate> {
			Coordinate {49.2, -112.94},
		};
		check(read_corners(contents), expected);
	}
}
