#include "Location.hpp"

#include "doctest.h"

#include <list>
#include <vector>

using namespace thalweg;

TEST_SUITE("LocationTest")
{
	TEST_CASE("to_coordinates works on vectors")
	{
		auto const contents = std::vector<Location> {
			Location {Coordinate {0, 0}, 123},
			Location {Coordinate {1, 1}, 321},
		};
		auto const expected = std::vector<Coordinate> {
			Coordinate {0, 0},
			Coordinate {1, 1},
		};
		CHECK(to_coordinates(contents) == expected);
	}

	TEST_CASE("to_coordinates works on lists")
	{
		auto const contents = std::list<Location> {
			Location {Coordinate {0, 0}, 123},
			Location {Coordinate {1, 1}, 321},
		};
		auto const expected = std::list<Coordinate> {
			Coordinate {0, 0},
			Coordinate {1, 1},
		};
		CHECK(to_coordinates(contents) == expected);
	}
}
