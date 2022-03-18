#pragma once

#include <iostream>

namespace thalweg
{
	struct Coordinate
	{
		double latitude;
		double longitude;
	};
	auto distance_between(Coordinate const& lhs, Coordinate const& rhs) -> double;

	auto operator==(Coordinate const& lhs, Coordinate const& rhs) -> bool;
	auto operator<<(std::ostream& os, Coordinate const& value) -> std::ostream&;
}
