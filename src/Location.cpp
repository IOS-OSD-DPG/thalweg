#include "Location.hpp"

#include <unordered_set>

namespace thalweg
{
auto Location::coordinates(Location const& loc) -> Coordinate
{
	return loc.coord;
}

auto shrink(std::vector<Location> data, unsigned resolution) -> std::vector<Location>
{
	std::vector<Location> out;
	std::unordered_set<Location> visited;

	for (auto const& location : data)
	{
		// only visit locations once
		if (visited.count(location) > 0)
			continue;

		std::vector<Location> neighbors;
		std::copy_if(
			data.begin(),
			data.end(),
			std::back_inserter(neighbors),
			[&](Location const& elem){ return distance_between(location.coord, elem.coord) < resolution; });
		auto const iter = std::max_element(
			neighbors.begin(),
			neighbors.end(),
			// comp must return true if lhs is *less* than rhs
			[](Location const& lhs, Location const& rhs){ return lhs.depth < rhs.depth; });
		out.push_back(*iter);

		visited.insert(neighbors.begin(), neighbors.end());
	}

	return out;
}

auto operator==(Location const& lhs, Location const& rhs) -> bool
{
	return lhs.coord == rhs.coord && lhs.depth == rhs.depth;
}

auto operator<<(std::ostream& os, Location const& value) -> std::ostream&
{
	os << "Location:{coord:" << value.coord << ", depth:" << value.depth << "}";
	return os;
}
} // namespace thalweg
