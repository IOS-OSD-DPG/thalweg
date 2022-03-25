#include "Location.hpp"

#include "DisplayVector.hpp"

#include "doctest.h"

#include <list>
#include <vector>

using namespace thalweg;

TEST_SUITE("LocationTest")
{
	TEST_CASE("to_coordinates works on vectors")
	{
		auto const contents = std::vector<Location> {
			Location {CoordinatePair {0, 0}, 123},
			Location {CoordinatePair {1, 1}, 321},
		};
		auto const expected = std::vector<CoordinatePair> {
			CoordinatePair {0, 0},
			CoordinatePair {1, 1},
		};
		CHECK(to_coordinates(contents) == expected);
	}

	TEST_CASE("to_depths works on vectors")
	{
		auto const contents = std::vector<Location> {
			Location {CoordinatePair {0, 0}, 123},
			Location {CoordinatePair {1, 1}, 321},
		};
		auto const expected = std::vector<double> {123, 321};
		CHECK(to_depths(contents) == expected);
	}

	TEST_CASE("Location can be hashed")
	{
		auto const hasher = std::hash<Location>();
		CHECK(hasher(Location{{0, 0}, 0}) == hasher(Location{{0, 0}, 0}));
		CHECK(hasher(Location{{0, 0}, 0}) != hasher(Location{{1, 0}, 0}));
	}

	TEST_CASE("max_depth_of returns the deepest value")
	{
		auto const data = std::vector<Location> {
			Location {CoordinatePair{0, 0}, 0},
			Location {CoordinatePair{1, 0}, 1},
			Location {CoordinatePair{0, 1}, 2},
			Location {CoordinatePair{1, 1}, 3},
			Location {CoordinatePair{-1, 0}, 4},
			Location {CoordinatePair{0, -1}, 5},
			Location {CoordinatePair{-1, -1}, 6},
		};
		CHECK(max_depth_of(data) == doctest::Approx(6.0));
	}

	TEST_CASE("shrink shrinks its input, maintaining the deepest point")
	{
		auto const data = std::vector<Location> {
			Location {CoordinatePair{0, 0}, 50},
			Location {CoordinatePair{0.01, 0.01}, 100},
			Location {CoordinatePair{-0.01, -0.01}, 50},
		};
		auto const deepest_point = max_depth_of(data);

		auto const result = shrink(data, 10000);
		auto const new_deepest_point = max_depth_of(result);
		CHECK(result.size() < data.size());
		CHECK(new_deepest_point == deepest_point);
	}

	TEST_CASE("distance_between can use Location objects as well as CoordinatePair objects")
	{
		auto const point1 = Location{CoordinatePair{49, -122}, 100};
		auto const point2 = Location{CoordinatePair{49, -123}, 100};
		auto const expected = doctest::Approx(distance_between(point1, point2));
		CHECK(distance_between(point1, point2) == expected);
		CHECK(distance_between(point1.coord, point2) == expected);
		CHECK(distance_between(point1, point2.coord) == expected);
	}
}
