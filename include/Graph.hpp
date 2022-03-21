#pragma once

#include "Location.hpp"

#include <vector>

namespace thalweg
{
class Graph
{
public:
	Graph(std::vector<Location>, unsigned);

	auto contains(Coordinate) const -> bool;

	auto adjacent(Coordinate, Coordinate) const -> bool;

	auto weight(Coordinate) const -> double;

	auto shortest_path(Coordinate const&, Coordinate const&) const -> std::vector<Location>;

private:
	using DataIterator = std::vector<Location>::const_iterator;

	auto find(Coordinate) const -> DataIterator;

	std::vector<Location> data;
	unsigned resolution;
	double max_depth;
};

auto shortest_path(Graph, Coordinate, Coordinate) -> std::vector<Location>;
} // namespace thalweg
