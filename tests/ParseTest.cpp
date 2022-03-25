#include "Parse.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("ParseTest")
{

	TEST_CASE("parse_dms_latitude returns 0 at equator")
	{
		CHECK(parse_dms_latitude("0-0-0.0N") == Latitude(Coordinate(0, 0, 0.0), true));
		CHECK(parse_dms_latitude("0-0-0.0S") == Latitude(Coordinate(0, 0, 0.0), false));
	}

	TEST_CASE("parse_dms_latitude returns correct decimal conversion")
	{
		CHECK(parse_dms_latitude("49-12-00.000N") == Latitude(Coordinate(49, 12, 0.0), true));
	}

	TEST_CASE("parse_dms_latitude rejects non-conformant values")
	{
		CHECK_THROWS(parse_dms_latitude("not a coordinate"));
	}

	TEST_CASE("parse_dms_latitude rejects unexpected direction markers")
	{
		CHECK_THROWS(parse_dms_latitude("0-0-0.0E"));
	}

	TEST_CASE("parse_dms_longitude returns 0 at meridian")
	{
		CHECK(parse_dms_longitude("0-0-0.0E") == Longitude(Coordinate(0, 0, 0.0), true));
		CHECK(parse_dms_longitude("0-0-0.0W") == Longitude(Coordinate(0, 0, 0.0), false));
	}

	TEST_CASE("parse_dms_longitude returns correct decimal conversion")
	{
		CHECK(parse_dms_longitude("112-56-24.360W") == Longitude(Coordinate(112, 56, 24.36), false));
	}

	TEST_CASE("parse_dms_longitude rejects non-conformant values")
	{
		CHECK_THROWS(parse_dms_longitude("not a coordinate"));
	}

	TEST_CASE("parse_dms_longitude rejects unexpected direction markers")
	{
		CHECK_THROWS(parse_dms_longitude("0-0-0.0N"));
	}

	TEST_CASE("parse_depth converts value to double")
	{
		CHECK(parse_depth("0.0") == doctest::Approx(0.0));
		CHECK(parse_depth("1.0") == doctest::Approx(1.0));
		CHECK(parse_depth("-1.0") == doctest::Approx(-1.0));
	}

	TEST_CASE("parse_depth throws on non-number values")
	{
		CHECK_THROWS(parse_depth("not a number"));
	}
}
