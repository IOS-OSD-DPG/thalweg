#pragma once

#include "Coordinate.hpp"

#include <string>

namespace thalweg
{
auto parse_dms_latitude(std::string const&) -> Latitude;
auto parse_dms_longitude(std::string const&) -> Longitude;
auto parse_depth(std::string const&) -> double;
} // namespace thalweg
