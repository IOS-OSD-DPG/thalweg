#pragma once

#include "Coordinate.hpp"
#include "Location.hpp"

#include <vector>

namespace thalweg
{
auto read_data(std::istream&) -> std::vector<Location>;
auto read_corners(std::istream&) -> std::vector<CoordinatePair>;
} // namespace thalweg
