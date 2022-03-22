#include "Coordinate.hpp"

#include <algorithm>
#include <cmath>
#include <functional>

namespace thalweg
{
namespace
{
// global average
double constexpr earth_radius_m = 6.371e6;

// pi is only defined properly in C++20 apparently
double constexpr pi = 3.14159;

auto haversine(double delta) -> double
{
	auto const to_square = std::sin(delta / 2);
	return to_square * to_square;
}
} // namespace

auto distance_between(Coordinate const& lhs, Coordinate const& rhs) -> double
{
	// the following formula assumes that lhs and rhs are different
	if (lhs == rhs)
		return 0.0;
	// haversine formula taken from https://movable-type.co.uk/scripts/latlong.html and
	// https://en.wikipedia.org/wiki/Haversine_formula
	auto constexpr to_radians = pi / 180;

	// phi denotes latitude in radians, lambda denotes longitude in radians
	auto const phi_left = lhs.latitude * to_radians;
	auto const phi_right = rhs.latitude * to_radians;

	auto const delta_phi = (rhs.latitude - lhs.latitude) * to_radians;
	auto const delta_lambda = (rhs.longitude - lhs.longitude) * to_radians;

	auto const hav_phi = haversine(delta_phi);
	auto const hav_lambda = haversine(delta_lambda);
	// not sure if the resolution concerns with cosine are significant enough in this case
	// auto const coefficient = std::cos(phi_left) * std::cos(phi_right);
	auto const coefficient = 1 - haversine(phi_left - phi_right) - haversine(phi_left + phi_right);
	auto const hav_theta = hav_phi + coefficient * hav_lambda;

	auto const angular_distance = 2 * std::asin(std::sqrt(hav_theta));
	auto const distance_m = earth_radius_m * angular_distance;

	return distance_m;
}

auto operator==(Coordinate const& lhs, Coordinate const& rhs) -> bool
{
	return lhs.latitude == rhs.latitude && lhs.longitude == rhs.longitude;
}

auto operator!=(Coordinate const& lhs, Coordinate const& rhs) -> bool
{
	return !operator==(lhs, rhs);
}

auto operator<<(std::ostream& os, Coordinate const& value) -> std::ostream&
{
	os << "Coordinate{latitude:" << value.latitude << ", longitude:" << value.longitude << "}";
	return os;
}
} // namespace thalweg
