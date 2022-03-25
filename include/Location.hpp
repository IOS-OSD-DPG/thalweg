#pragma once

#include "Coordinate.hpp"

#include <algorithm>
#include <iostream>
#include <vector>

namespace thalweg
{
struct Location
{
	CoordinatePair coord;
	double depth;

	static auto coordinates(Location const&) -> CoordinatePair;
	static auto depth_of(Location const&) -> double;
};

auto distance_between(Location const& lhs, CoordinatePair const& rhs) -> double;
auto distance_between(CoordinatePair const& lhs, Location const& rhs) -> double;
auto distance_between(Location const& lhs, Location const& rhs) -> double;

auto to_coordinates(std::vector<Location> const&) -> std::vector<CoordinatePair>;
auto to_depths(std::vector<Location> const&) -> std::vector<double>;

auto max_depth_of(std::vector<Location> const&) -> double;

auto shrink(std::vector<Location>, unsigned) -> std::vector<Location>;

auto operator==(Location const&, Location const&) -> bool;
auto operator<<(std::ostream&, Location const&) -> std::ostream&;
} // namespace thalweg

template <>
struct std::hash<thalweg::Location>
{
	auto operator()(thalweg::Location const& point) const noexcept -> size_t
	{
		return std::hash<thalweg::CoordinatePair>()(point.coord) ^ std::hash<double>()(point.depth);
	}
};
