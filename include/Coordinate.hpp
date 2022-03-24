#pragma once

#include <algorithm>
#include <cmath>
#include <execution>
#include <iostream>

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
	auto const closest = std::min_element(
		std::execution::par_unseq,
		begin,
		end,
		[&](Coordinate const& lhs, Coordinate const& rhs)
		{
			return distance_between(point, lhs) < distance_between(point, rhs);
		});

	if (closest == end)
		throw std::runtime_error("empty collection");

	return *closest;
}

// clang requires the allocator be acknowledged in the template
template<typename Allocator, template <typename, typename> typename Collection>
auto closest_point(Coordinate const& point, Collection<Coordinate, Allocator> const& collection) -> Coordinate
{
	return closest_point(point, std::begin(collection), std::end(collection));
}

auto operator==(Coordinate const&, Coordinate const&) -> bool;
// only necessary until C++20
auto operator!=(Coordinate const&, Coordinate const&) -> bool;
auto operator<<(std::ostream&, Coordinate const&) -> std::ostream&;
auto coordinate_hash(Coordinate const&) -> size_t;
} // namespace thalweg

template <>
struct std::hash<thalweg::Coordinate>
{
	auto operator()(thalweg::Coordinate const& point) const noexcept -> size_t
	{
		auto const hasher = std::hash<double>{};
		return hasher(point.latitude) ^ hasher(point.longitude);
	}
};
