#pragma once

#include "Location.hpp"
#include "Search.hpp"

#include <vector>

namespace thalweg
{
class Graph
{
public:
	Graph(std::vector<Location>, unsigned);

	auto contains(CoordinatePair) const -> bool;

	auto adjacent(CoordinatePair, CoordinatePair) const -> bool;

	auto weight(CoordinatePair) const -> double;

	auto shortest_path(CoordinatePair const&, CoordinatePair const&) const -> std::vector<Location>;

private:
	using DataIterator = std::vector<Location>::const_iterator;

	auto find(CoordinatePair) const -> DataIterator;

	std::vector<Location> data;
	SearchTree search_tree;
	unsigned resolution;
	double max_depth;
};
} // namespace thalweg
