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

	TEST_CASE("Location can be hashed")
	{
		auto const hasher = std::hash<Location>();
		CHECK(hasher(Location{{0, 0}, 0}) == hasher(Location{{0, 0}, 0}));
		CHECK(hasher(Location{{0, 0}, 0}) != hasher(Location{{1, 0}, 0}));
	}

	TEST_CASE("shrink shrinks its input, maintaining the deepest point")
	{
		auto const data = std::vector<Location> {
			Location {Coordinate{0, 0}, 50},
			Location {Coordinate{0.01, 0.01}, 100},
			Location {Coordinate{-0.01, -0.01}, 50},
		};
		auto const deepest_point = *std::max_element(
			data.begin(),
			data.end(),
			[](auto const& lhs, auto const& rhs){ return lhs.depth < rhs.depth; });

		auto const result = shrink(data, 10000);
		auto const new_deepest_point = *std::max_element(
			data.begin(),
			data.end(),
			[](auto const& lhs, auto const& rhs){ return lhs.depth < rhs.depth; });
		CHECK(result.size() < data.size());
		CHECK(new_deepest_point == deepest_point);
	}
}
