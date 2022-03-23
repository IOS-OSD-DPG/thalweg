#pragma once

#include "Coordinate.hpp"

#include <algorithm>
#include <iostream>

namespace thalweg
{
struct Location
{
	Coordinate coord;
	double depth;

	static auto coordinates(Location const&) -> Coordinate;
};

template<template <typename> typename Allocator, template <typename,typename> typename Container>
auto to_coordinates(
	Container<Location, Allocator<Location>> const& in)
	-> Container<Coordinate, Allocator<Coordinate>>
{
	Container<Coordinate, Allocator<Coordinate>> out;
	std::transform(
		std::begin(in),
		std::end(in),
		std::back_inserter(out),
		&Location::coordinates);
	return out;
}

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
