#include "Parse.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("ParseTest")
{

TEST_CASE("parse_dms_latitude returns 0 at equator")
{
	CHECK(parse_dms_latitude("0-0-0.0N") == doctest::Approx(0.0));
	CHECK(parse_dms_latitude("0-0-0.0S") == doctest::Approx(0.0));
}

TEST_CASE("parse_dms_latitude returns correct decimal conversion")
{
	CHECK(parse_dms_latitude("49-12-00.000N") == doctest::Approx(49.2));
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
	CHECK(parse_dms_longitude("0-0-0.0E") == doctest::Approx(0.0));
	CHECK(parse_dms_longitude("0-0-0.0W") == doctest::Approx(0.0));
}

TEST_CASE("parse_dms_longitude returns correct decimal conversion")
{
	CHECK(parse_dms_longitude("112-56-24.360W") == doctest::Approx(-112.94));
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
