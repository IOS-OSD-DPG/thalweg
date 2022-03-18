#include "Coordinate.hpp"

#include <cmath>
#include <numbers>

namespace
{

double constexpr earth_radius_m = 6.3781e6;

auto haversine(double delta) -> double
{
	auto const to_square = std::sin(delta / 2);
	return to_square * to_square;
}

}

namespace thalweg
{

auto distance_between(Coordinate const& lhs, Coordinate const& rhs) -> double
{
	// the following formula assumes that lhs and rhs are different
	if (lhs == rhs)
		return 0.0;
	// haversine formula taken from https://movable-type.co.uk/scripts/latlong.html and https://en.wikipedia.org/wiki/Haversine_formula
	auto const to_radians = std::numbers::pi / 180;

	// phi denotes latitude in radians, lambda denotes longitude in radians
	auto const phi_left = lhs.latitude * to_radians;
	auto const phi_right = rhs.latitude * to_radians;

	auto const delta_phi = (rhs.latitude - lhs.longitude) * to_radians;
	auto const delta_lambda = (rhs.longitude - lhs.longitude) * to_radians;

	auto const hav_phi = haversine(delta_phi);
	auto const hav_lambda = haversine(delta_lambda);
	auto const hav_theta = hav_phi + std::cos(phi_left) * std::cos(phi_right) * hav_lambda;

	auto const angular_distance = 2 * std::atan2(std::sqrt(hav_theta), std::sqrt(1 - hav_theta));
	auto const distance_m = earth_radius_m * angular_distance;

	return distance_m;
}

auto closest_point(Coordinate const& point, std::vector<Coordinate> const& collection) -> Coordinate
{
	if (collection.size() == 0)
		throw std::runtime_error("empty collection");

	auto best_distance = std::numeric_limits<double>::infinity();
	auto best_point = point;

	for (auto const& elem : collection)
	{
		auto const new_distance = distance_between(point, elem);
		if (new_distance < best_distance)
		{
			best_distance = new_distance;
			best_point = elem;
		}
	}

	return best_point;
}

auto operator==(Coordinate const& lhs, Coordinate const& rhs) -> bool
{
	return lhs.latitude == rhs.latitude && lhs.longitude == rhs.longitude;
}

auto operator<<(std::ostream& os, Coordinate const& value) -> std::ostream&
{
	os << "Coordinate{latitude:" << value.latitude << ", longitude:" << value.longitude << "}";
	return os;
}

}
