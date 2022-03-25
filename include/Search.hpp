#pragma once

#include "Coordinate.hpp"

#include <unordered_set>
#include <map>
#include <vector>

namespace thalweg
{
class SearchTree
{
public:
	explicit SearchTree(std::vector<CoordinatePair> const& input);

	auto neighbors(CoordinatePair const&) const -> std::unordered_set<CoordinatePair>;

	auto closest_point(CoordinatePair const&) const -> CoordinatePair;

private:
	using Index = std::pair<int, int>;
	using Map = std::map<Index, size_t>;
	using MapIter = Map::const_iterator;
	using Set = std::unordered_set<CoordinatePair>;

	auto insert(CoordinatePair const&) -> void;

	auto find(CoordinatePair const&) const -> MapIter;
	auto find(Index) const -> MapIter;

	Map bucket_index_map;
	std::vector<Set> location_buckets;
};
} // namespace thalweg
