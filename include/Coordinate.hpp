#pragma once

#include <algorithm>
#include <cmath>
#include <iostream>

namespace thalweg
{
struct Coordinate
{
	unsigned degrees;
	unsigned minutes;
	double seconds;

	Coordinate() = default;
	explicit Coordinate(double);
	Coordinate(unsigned degrees, unsigned minutes, double seconds)
		: degrees(degrees), minutes(minutes), seconds(seconds)
	{
	}

	virtual explicit operator double() const;
};

struct Latitude : public Coordinate
{
	bool is_north;

	Latitude() = default;
	explicit Latitude(double degrees)
		: Coordinate(std::abs(degrees))
		, is_north(degrees >= 0)
	{
	}
	Latitude(Coordinate coordinate, bool is_north)
		: Coordinate(coordinate)
		, is_north(is_north)
	{
	}

	explicit operator double() const override;
};

struct Longitude : public Coordinate
{
	bool is_east;

	Longitude() = default;
	explicit Longitude(double degrees)
		: Coordinate(std::abs(degrees))
		, is_east(degrees >= 0)
	{
	}
	Longitude(Coordinate coordinate, bool is_east)
		: Coordinate(coordinate)
		, is_east(is_east)
	{
	}

	explicit operator double() const override;
};

struct CoordinatePair
{
	Latitude latitude;
	Longitude longitude;

	CoordinatePair() = default;
	CoordinatePair(double lat, double lon)
		: latitude(lat)
		, longitude(lon)
	{
	}
	CoordinatePair(Latitude lat, Longitude lon)
		: latitude(lat)
		, longitude(lon)
	{
	}
};

auto distance_between(CoordinatePair const&, CoordinatePair const&) -> double;

template<typename Iter>
auto closest_point(CoordinatePair const& point, Iter begin, Iter end) -> CoordinatePair
{
	auto const closest = std::min_element(
		begin,
		end,
		[&](CoordinatePair const& lhs, CoordinatePair const& rhs)
		{
			return distance_between(point, lhs) < distance_between(point, rhs);
		});

	if (closest == end)
		throw std::runtime_error("empty collection");

	return *closest;
}

// clang requires the allocator be acknowledged in the template
template<typename Allocator, template <typename, typename> typename Collection>
auto closest_point(CoordinatePair const& point, Collection<CoordinatePair, Allocator> const& collection) -> CoordinatePair
{
	return closest_point(point, std::begin(collection), std::end(collection));
}

auto operator==(Coordinate const&, Coordinate const&) -> bool;
// only necessary until C++20
auto operator!=(Coordinate const&, Coordinate const&) -> bool;
auto operator<<(std::ostream&, Coordinate const&) -> std::ostream&;

auto operator==(Latitude const&, Latitude const&) -> bool;
// only necessary until C++20
auto operator!=(Latitude const&, Latitude const&) -> bool;
auto operator<<(std::ostream&, Latitude const&) -> std::ostream&;

auto operator==(Longitude const&, Longitude const&) -> bool;
// only necessary until C++20
auto operator!=(Longitude const&, Longitude const&) -> bool;
auto operator<<(std::ostream&, Longitude const&) -> std::ostream&;

auto operator==(CoordinatePair const&, CoordinatePair const&) -> bool;
// only necessary until C++20
auto operator!=(CoordinatePair const&, CoordinatePair const&) -> bool;
auto operator<<(std::ostream&, CoordinatePair const&) -> std::ostream&;
} // namespace thalweg

template <>
struct std::hash<thalweg::Coordinate>
{
	auto operator()(thalweg::Coordinate const& point) const noexcept -> size_t
	{
		std::hash<int> const int_hasher;
		std::hash<double> const double_hasher;
		std::hash<bool> const bool_hasher;
		return int_hasher(point.degrees)
			^ int_hasher(point.minutes)
			^ double_hasher(point.seconds);
	}
};

template <>
struct std::hash<thalweg::Latitude>
{
	auto operator()(thalweg::Latitude const& point) const noexcept -> size_t
	{
		std::hash<thalweg::Coordinate> const hasher;
		std::hash<bool> const bool_hasher;
		return hasher(point) ^ bool_hasher(point.is_north);
	}
};

template <>
struct std::hash<thalweg::Longitude>
{
	auto operator()(thalweg::Longitude const& point) const noexcept -> size_t
	{
		std::hash<thalweg::Coordinate> const hasher;
		std::hash<bool> const bool_hasher;
		return hasher(point) ^ bool_hasher(point.is_east);
	}
};

template <>
struct std::hash<thalweg::CoordinatePair>
{
	auto operator()(thalweg::CoordinatePair const& point) const noexcept -> size_t
	{
		auto const hasher = std::hash<thalweg::Coordinate>{};
		return hasher(point.latitude) ^ hasher(point.longitude);
	}
};
