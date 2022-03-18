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
	auto operator==(Location const&, Location const&) -> bool;
	auto operator<<(std::ostream&, Location const&) -> std::ostream&;
}
