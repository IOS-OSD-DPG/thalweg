#include "Parse.hpp"
#include "Utils.hpp"

#include <algorithm>
#include <cstdlib>
#include <stdexcept>
#include <vector>

namespace thalweg
{
namespace
{

auto get_dms_coord(std::string const& value, int bound, bool negate) -> double
{
	double out;
	auto const split_vals = thalweg::utils::split(value, '-');
	if (split_vals.size() != 3)
		throw std::runtime_error(value + " has an unexpected number of sections");

	int const degrees = std::stoi(split_vals[0]);
	if (degrees < -bound || degrees > bound)
		throw std::runtime_error(value + " has a degree value outside the expected bounds");

	int const minutes = std::stoi(split_vals[1]);
	if (minutes < 0 || minutes > 60)
		throw std::runtime_error(value + " has a minute value outside the expected bounds");

	int const seconds = std::stod(split_vals[2]);
	if (seconds < 0.0 || seconds > 60.0)
		throw std::runtime_error(value + " has a second value outside the expected bounds");

	out = degrees + (minutes / 60.0) + (seconds / 3600.0);

	return negate ? -out : out;
}

auto dash_only_at_start(std::string const& value) -> bool
{
	bool const starts_with_dash = value.front() == '-';
	auto const total = std::count(value.begin(), value.end(), '-');
	if (starts_with_dash)
		return total == 1;
	else
		return total == 0;
}

auto is_legal_in_number(char c) -> bool
{
	return c == '-' || c == '.' || (c >= '0' && c <= '9');
}
} // namespace

auto parse_dms_latitude(std::string const& latitude) -> double
{
	auto const direction = latitude.back();
	auto const trimmed = latitude.substr(0, latitude.size() - 1);
	switch (direction)
	{
	case 'n':
	case 'N':
		return get_dms_coord(trimmed, 90, false);
	case 's':
	case 'S':
		return get_dms_coord(trimmed, 90, true);
	default:
		throw std::runtime_error(latitude + " contains unexpected direction marker " + direction);
	}
}

auto parse_dms_longitude(std::string const& longitude) -> double
{
	auto const direction = longitude.back();
	auto const trimmed = longitude.substr(0, longitude.size() - 1);
	switch (direction)
	{
	case 'e':
	case 'E':
		return get_dms_coord(trimmed, 180, false);
	case 'w':
	case 'W':
		return get_dms_coord(trimmed, 180, true);
	default:
		throw std::runtime_error(longitude + " contains unexpected directon marker " + direction);
	}
}

auto parse_depth(std::string const& value) -> double
{
	bool const all_legal = std::all_of(value.begin(), value.end(), is_legal_in_number);
	bool const only_one_decimal = std::count(value.begin(), value.end(), '.') <= 1;
	if (!all_legal || !only_one_decimal || !dash_only_at_start(value))
		throw std::runtime_error(value + " is not a legal double");
	return std::stod(value);
}

} // namespace thalweg
