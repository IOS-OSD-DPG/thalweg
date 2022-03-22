#pragma once

#include <cmath>
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

template<typename Iter>
auto closest_point(Coordinate const& point, Iter begin, Iter end) -> Coordinate
{
	if (begin == end)
		throw std::runtime_error("empty collection");

	auto best_distance = std::numeric_limits<double>::infinity();
	auto best_point = point;

	for (auto iter = begin; iter != end; ++iter)
	{
		auto const new_distance = distance_between(point, *iter);
		if (new_distance < best_distance)
		{
			best_distance = new_distance;
			best_point = *iter;
		}
	}

	return best_point;
}

template<template <typename> typename Collection>
auto closest_point(Coordinate const& point, Collection<Coordinate> const& collection) -> Coordinate
{
	return closest_point(point, std::begin(collection), std::end(collection));
}

auto operator==(Coordinate const&, Coordinate const&) -> bool;
// only necessary until C++20
auto operator!=(Coordinate const&, Coordinate const&) -> bool;
auto operator<<(std::ostream&, Coordinate const&) -> std::ostream&;
auto coordinate_hash(Coordinate const&) -> size_t;
} // namespace thalweg
