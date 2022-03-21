#include "Read.hpp"

#include "Parse.hpp"
#include "Utils.hpp"

#include <algorithm>
#include <array>
#include <stdexcept>

namespace thalweg
{

size_t constexpr buf_size = 1024;

auto read_data(std::istream& contents) -> std::vector<Location>
{
	std::vector<Location> out;

	for (std::array<char, buf_size> buffer; contents.getline(buffer.data(), buf_size);)
	{
		auto line = std::string(buffer.data());
		if (std::count(line.begin(), line.end(), '"') != 0)
			continue;
		auto vals = utils::split(line, ' ');
		if (vals.size() != 3)
			throw std::runtime_error(line + " has an unexpected number of values");
		auto lat = parse_dms_latitude(vals[0]);
		auto lon = parse_dms_longitude(vals[1]);
		auto depth = parse_depth(vals[2]);
		out.emplace_back(Coordinate {lat, lon}, depth);
	}

	return out;
}

auto read_corners(std::istream& contents) -> std::vector<Coordinate>
{
	std::vector<Coordinate> out;

	for (std::array<char, buf_size> buffer; contents.getline(buffer.data(), buf_size);)
	{
		auto line = std::string(buffer.data());
		if (std::count(line.begin(), line.end(), '"') != 0)
			continue;
		auto vals = utils::split(line, ' ');
		if (vals.size() != 2)
			throw std::runtime_error(line + " has an unexpected number of values");
		auto lat = parse_dms_latitude(vals[0]);
		auto lon = parse_dms_longitude(vals[1]);
		out.emplace_back(lat, lon);
	}

	return out;
}

} // namespace thalweg
