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

	for (std::array<char, buf_size> buffer; contents.getline(buffer.data(), buf_size); )
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
		out.push_back(Location{Coordinate{lat, lon}, depth});
	}

	return out;
}

auto read_corners(std::istream& contents) -> std::vector<Coordinate>
{
	std::vector<Coordinate> out;

	for (std::array<char, buf_size> buffer; contents.getline(buffer.data(), buf_size); )
	{
		auto line = std::string(buffer.data());
		if (std::count(line.begin(), line.end(), '"') != 0)
			continue;
		auto vals = utils::split(line, ' ');
		if (vals.size() != 2)
			throw std::runtime_error(line + " has an unexpected number of values");
		auto lat = parse_dms_latitude(vals[0]);
		auto lon = parse_dms_longitude(vals[1]);
		out.push_back(Coordinate{lat, lon});
	}

	return out;
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

auto operator==(Location const& lhs, Location const& rhs) -> bool
{
	return lhs.coord == rhs.coord && lhs.depth == rhs.depth;
}

auto operator<<(std::ostream& os, Location const& value) -> std::ostream&
{
	os << "Location:{coord:" << value.coord << ", depth:" << value.depth << "}";
	return os;
}

}
