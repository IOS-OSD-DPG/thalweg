#include "Coordinate.hpp"

#include "doctest.h"

#include <vector>

using namespace thalweg;

TEST_SUITE("CoordinateTest")
{

	TEST_CASE("distance_between point and itself is 0")
	{
		auto const point = CoordinatePair {49, -122};
		CHECK(distance_between(point, point) == doctest::Approx(0.0));
	}

	TEST_CASE("distance_between is always positive")
	{
		auto const point1 = CoordinatePair {0, 0};
		auto const point2 = CoordinatePair {1, 1};
		CHECK(distance_between(point1, point2) > 0.0);
		CHECK(distance_between(point2, point1) > 0.0);
	}

	TEST_CASE("distance_between is reflexive")
	{
		auto const point1 = CoordinatePair {0, 0};
		auto const point2 = CoordinatePair {1, 1};
		CHECK(distance_between(point1, point2) == distance_between(point2, point1));
	}

	TEST_CASE("distance_between (0, 0) and (1, 1) is ~157km")
	{
		auto const point1 = CoordinatePair {0, 0};
		auto const point2 = CoordinatePair {1, 1};
		CHECK((distance_between(point1, point2) / 1000) == doctest::Approx(157).epsilon(0.002));
	}

	TEST_CASE("distance_between (49.4678, -122.883) and (49.2989, -122.94) is ~19km")
	{
		auto const point1 = CoordinatePair {49.4678, -122.883};
		auto const point2 = CoordinatePair {49.2989, -122.94};
		CHECK((distance_between(point1, point2) / 1000) == doctest::Approx(19).epsilon(0.02));
	}

	TEST_CASE("distance_between (0, 0) and all corners of a square are roughly equal")
	{
		auto const middle = CoordinatePair {0, 0};
		auto const top_left = CoordinatePair {1, -1};
		auto const top_right = CoordinatePair {1, 1};
		auto const bot_left = CoordinatePair {-1, -1};
		auto const bot_right = CoordinatePair {-1, 1};

		CHECK(distance_between(middle, top_left) == distance_between(middle, top_right));
		CHECK(distance_between(middle, top_left) == distance_between(middle, bot_left));
		CHECK(distance_between(middle, top_left) == distance_between(middle, bot_right));
		CHECK(distance_between(middle, top_right) == distance_between(middle, bot_left));
		CHECK(distance_between(middle, top_right) == distance_between(middle, bot_right));
		CHECK(distance_between(middle, bot_left) == distance_between(middle, bot_right));
	}

	TEST_CASE("closest_point rejects empty collection")
	{
		CHECK_THROWS(closest_point(CoordinatePair {0, 0}, std::vector<CoordinatePair>{}));
	}

	TEST_CASE("closest_point returns a value in the collection")
	{
		auto const point = CoordinatePair {0, 0};
		auto const collection = std::vector<CoordinatePair> {
			CoordinatePair {1, 1},
		};
		CHECK(closest_point(point, collection) == CoordinatePair {1, 1});
	}

	TEST_CASE("closest_point returns obviously better value")
	{
		auto const point = CoordinatePair {0, 0};
		auto const collection = std::vector<CoordinatePair> {
			CoordinatePair {1, 1},
			CoordinatePair {2, 2},
		};
		CHECK(closest_point(point, collection) == CoordinatePair {1, 1});
	}

	TEST_CASE("closest_point chooses first value with same distance")
	{
		auto const point = CoordinatePair {0, 0};
		auto const collection = std::vector<CoordinatePair> {
			CoordinatePair { 1, 1},
			CoordinatePair {-1, 1},
		};
		CHECK(closest_point(point, collection) == CoordinatePair {1, 1});
	}

	TEST_CASE("CoordinatePair can be hashed")
	{
		auto const hasher = std::hash<CoordinatePair>();
		CHECK(hasher(CoordinatePair{0, 0}) == hasher(CoordinatePair{0, 0}));
		CHECK(hasher(CoordinatePair{0, 0}) != hasher(CoordinatePair{1, 0}));
	}

	TEST_CASE("Coordinate operator double does the right thing")
	{
		CHECK(double(Coordinate(0.0)) == doctest::Approx(0.0));
		CHECK(double(Coordinate(0.5)) == doctest::Approx(0.5));
		CHECK(double(Coordinate(1.0)) == doctest::Approx(1.0));
		CHECK(double(Coordinate(1.5)) == doctest::Approx(1.5));
		CHECK(double(Coordinate(2.0)) == doctest::Approx(2.0));
		CHECK(double(Coordinate(2.5)) == doctest::Approx(2.5));
		CHECK(double(Coordinate(3.0)) == doctest::Approx(3.0));
		CHECK(double(Coordinate(3.5)) == doctest::Approx(3.5));
		CHECK(double(Coordinate(4.0)) == doctest::Approx(4.0));
	}
}
