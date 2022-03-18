#include "Coordinate.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("CoordinateTest")
{

TEST_CASE("distance_between point and itself is 0")
{
	auto const point = Coordinate{49, -122};
	CHECK(distance_between(point, point) == doctest::Approx(0.0));
}

TEST_CASE("distance_between is always positive")
{
	auto const point1 = Coordinate{0, 0};
	auto const point2 = Coordinate{1, 1};
	CHECK(distance_between(point1, point2) > 0.0);
	CHECK(distance_between(point2, point1) > 0.0);
}

TEST_CASE("distance_between is reflexive")
{
	auto const point1 = Coordinate{0, 0};
	auto const point2 = Coordinate{1, 1};
	CHECK(distance_between(point1, point2) == distance_between(point2, point1));
}

TEST_CASE("distance_between (0, 0) and (1, 1) is ~157km")
{
	auto const point1 = Coordinate{0, 0};
	auto const point2 = Coordinate{1, 1};
	CHECK((distance_between(point1, point2) / 1000) == doctest::Approx(157).epsilon(0.002));
}

TEST_CASE("distance_between (49.4678, -122.883) and (49.2989, -122.94) is ~19km")
{
	auto const point1 = Coordinate{49.4678, -122.883};
	auto const point2 = Coordinate{49.2989, -122.94};
	CHECK((distance_between(point1, point2) / 1000) == doctest::Approx(19).epsilon(0.02));
}

TEST_CASE("distance_between (0, 0) and all corners of a square are roughly equal")
{
	auto const middle = Coordinate{0, 0};
	auto const top_left = Coordinate{1, -1};
	auto const top_right = Coordinate{1, 1};
	auto const bot_left = Coordinate{-1, -1};
	auto const bot_right = Coordinate{-1, 1};

	CHECK(distance_between(middle, top_left) == distance_between(middle, top_right));
	CHECK(distance_between(middle, top_left) == distance_between(middle, bot_left));
	CHECK(distance_between(middle, top_left) == distance_between(middle, bot_right));
	CHECK(distance_between(middle, top_right) == distance_between(middle, bot_left));
	CHECK(distance_between(middle, top_right) == distance_between(middle, bot_right));
	CHECK(distance_between(middle, bot_left) == distance_between(middle, bot_right));
}

TEST_CASE("closest_point rejects empty collection")
{
	CHECK_THROWS(closest_point(Coordinate{0, 0}, {}));
}

TEST_CASE("closest_point returns a value in the collection")
{
	auto const point = Coordinate{0, 0};
	auto const collection = std::vector<Coordinate> {
		Coordinate{1, 1},
	};
	CHECK(closest_point(point, collection) == Coordinate{1, 1});
}

TEST_CASE("closest_point returns obviously better value")
{
	auto const point = Coordinate{0, 0};
	auto const collection = std::vector<Coordinate> {
		Coordinate{1, 1},
		Coordinate{2, 2},
	};
	CHECK(closest_point(point, collection) == Coordinate{1, 1});
}

TEST_CASE("closest_point chooses first value with same distance")
{
	auto const point = Coordinate{0, 0};
	auto const collection = std::vector<Coordinate> {
		Coordinate{1, 1},
		Coordinate{-1, 1},
	};
	CHECK(closest_point(point, collection) == Coordinate{1, 1});
}

}
