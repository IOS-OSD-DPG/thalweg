#pragma once

#include <iostream>
#include <vector>

namespace thalweg
{
	struct Coordinate
	{
		double latitude;
		double longitude;
	};
	auto operator==(Coordinate const& lhs, Coordinate const& rhs) -> bool;
	auto operator<<(std::ostream& os, Coordinate const& value) -> std::ostream&;

	struct Location
	{
		Coordinate coord;
		double depth;
	};
	auto operator==(Location const& lhs, Location const& rhs) -> bool;
	auto operator<<(std::ostream& os, Location const& value) -> std::ostream&;

	auto read_data(std::istream&) -> std::vector<Location>;
	auto read_corners(std::istream&) -> std::vector<Coordinate>;
}
