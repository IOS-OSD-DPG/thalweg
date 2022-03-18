#include "Coordinate.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("CoordinateTest")
{

TEST_CASE("distance_between point and itself is 0")
{
	auto point = Coordinate{49, -122};
	CHECK(distance_between(point, point) == doctest::Approx(0.0));
}

TEST_CASE("distance_between is always positive")
{
	auto point1 = Coordinate{0, 0};
	auto point2 = Coordinate{1, 1};
	CHECK(distance_between(point1, point2) > 0.0);
	CHECK(distance_between(point2, point1) > 0.0);
}

TEST_CASE("distance_between is reflexive")
{
	auto point1 = Coordinate{0, 0};
	auto point2 = Coordinate{1, 1};
	CHECK(distance_between(point1, point2) == distance_between(point2, point1));
}

TEST_CASE("distance_between (0, 0) and (1, 1) is ~157km")
{
	auto point1 = Coordinate{0, 0};
	auto point2 = Coordinate{1, 1};
	CHECK((distance_between(point1, point2) / 1000) == doctest::Approx(157).epsilon(1));
}

}
