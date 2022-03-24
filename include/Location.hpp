#pragma once

#include "Coordinate.hpp"

#include <algorithm>
#include <execution>
#include <iostream>
#include <vector>

namespace thalweg
{
struct Location
{
	Coordinate coord;
	double depth;

	static auto coordinates(Location const&) -> Coordinate;
	static auto depth_of(Location const&) -> double;
};

auto distance_between(Location const& lhs, Coordinate const& rhs) -> double;
auto distance_between(Coordinate const& lhs, Location const& rhs) -> double;
auto distance_between(Location const& lhs, Location const& rhs) -> double;

auto to_coordinates(std::vector<Location> const&) -> std::vector<Coordinate>;
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
		return std::hash<thalweg::Coordinate>()(point.coord) ^ std::hash<double>()(point.depth);
	}
};
