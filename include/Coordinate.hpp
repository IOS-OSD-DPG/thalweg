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

	auto distance_between(Coordinate const&, Coordinate const&) -> double;

	auto closest_point(Coordinate const&, std::vector<Coordinate> const&) -> Coordinate;

	auto operator==(Coordinate const&, Coordinate const&) -> bool;
	auto operator<<(std::ostream&, Coordinate const&) -> std::ostream&;
}
