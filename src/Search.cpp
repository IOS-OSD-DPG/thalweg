#include "Search.hpp"

namespace thalweg
{
namespace
{
auto bucket_id(CoordinatePair const& point) -> std::pair<int, int>
{
	return std::make_pair(point.latitude.neighborhood(), point.longitude.neighborhood());
}
} // namespace
SearchTree::SearchTree(std::vector<CoordinatePair> const& input)
{
	for (auto const& elem : input)
	{
		this->insert(elem);
	}
}

auto SearchTree::neighbors(CoordinatePair const& point) const -> std::unordered_set<CoordinatePair>
{
	std::unordered_set<CoordinatePair> out;
	auto const id = bucket_id(point);
	auto const neighboring_buckets = std::vector<std::pair<int, int>> {
		{id.first - 1, id.second - 1},
		{id.first, id.second - 1},
		{id.first + 1, id.second - 1},
		{id.first - 1, id.second},
		{id.first, id.second},
		{id.first + 1, id.second},
		{id.first - 1, id.second + 1},
		{id.first, id.second + 1},
		{id.first + 1, id.second + 1},
	};

	for (auto const& pair : neighboring_buckets)
	{
		auto const iter = find(pair);
		if (iter == bucket_index_map.end())
			continue;
		auto const& bucket = location_buckets[iter->second];
		out.insert(bucket.begin(), bucket.end());
	}

	return out;
}

auto SearchTree::closest_point(CoordinatePair const& point) const -> CoordinatePair
{
	std::vector<CoordinatePair> search_space;
	auto const neighbor_set = neighbors(point);
	if (neighbor_set.size() == 0)
	{
		// fall back to all coordinates
		for (auto const& bucket : location_buckets)
		{
			search_space.insert(search_space.end(), bucket.begin(), bucket.end());
		}
	}
	else
	{
		search_space.insert(search_space.end(), neighbor_set.begin(), neighbor_set.end());
	}
	return thalweg::closest_point(point, search_space);
}

auto SearchTree::insert(CoordinatePair const& point) -> void
{
	auto const id = bucket_id(point);
	auto const iter = find(id);
	size_t idx = 0;
	if (iter == bucket_index_map.end())
	{
		bucket_index_map[id] = location_buckets.size();
		idx = location_buckets.size();
		location_buckets.push_back(Set());
	}
	else
	{
		idx = iter->second;
	}
	location_buckets[idx].insert(point);
}

auto SearchTree::find(CoordinatePair const& point) const -> MapIter
{
	return find(bucket_id(point));
}

auto SearchTree::find(Index index) const -> MapIter
{
	return bucket_index_map.find(index);
}
} // namespace thalweg
