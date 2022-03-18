#pragma once

#include "Coordinate.hpp"

#include <iostream>

namespace thalweg
{
	struct Location
	{
		Coordinate coord;
		double depth;
	};
	auto operator==(Location const& lhs, Location const& rhs) -> bool;
	auto operator<<(std::ostream& os, Location const& value) -> std::ostream&;
}
