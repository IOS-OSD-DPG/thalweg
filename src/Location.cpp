#include "Location.hpp"

#include <execution>
#include <unordered_set>

namespace thalweg
{
auto Location::coordinates(Location const& loc) -> Coordinate
{
	return loc.coord;
}

auto Location::depth_of(Location const& loc) -> double
{
	return loc.depth;
}

auto distance_between(Location const& lhs, Coordinate const& rhs) -> double
{
	return distance_between(lhs.coord, rhs);
}
auto distance_between(Coordinate const& lhs, Location const& rhs) -> double
{
	return distance_between(lhs, rhs.coord);
}
auto distance_between(Location const& lhs, Location const& rhs) -> double
{
	return distance_between(lhs.coord, rhs.coord);
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
			std::execution::par_unseq,
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

auto to_coordinates(std::vector<Location> const& in) -> std::vector<Coordinate>
{
	std::vector<Coordinate> out(in.size());
	std::transform(
		std::execution::par_unseq,
		in.begin(),
		in.end(),
		out.begin(),
		&Location::coordinates);
	return out;
}

auto to_depths(std::vector<Location> const& in) -> std::vector<double>
{
	std::vector<double> out(in.size());
	std::transform(
		std::execution::par_unseq,
		in.begin(),
		in.end(),
		out.begin(),
		&Location::depth_of);
	return out;
}

auto max_depth_of(std::vector<Location> const& v) -> double
{
	return std::accumulate(
		v.begin(),
		v.end(),
		0.0,
		[](double acc, Location val) { return std::max(acc, val.depth); });
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
