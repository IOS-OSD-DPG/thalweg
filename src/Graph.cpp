#include "Graph.hpp"

#include <algorithm>
#include <numeric>
#include <unordered_map>
#include <unordered_set>

template <>
struct std::hash<thalweg::Coordinate>
{
	auto operator()(thalweg::Coordinate const& point) const noexcept -> size_t
	{
		auto const hasher = std::hash<double>{};
		return hasher(point.latitude) ^ hasher(point.longitude);
	}
};

namespace thalweg
{
namespace
{
template<typename K, typename V>
using Map = std::unordered_map<K, V>;

template<typename K>
using Set = std::unordered_set<K>;

auto max_depth_of(std::vector<thalweg::Location> const& v) -> double
{
	return std::accumulate(
		v.begin(),
		v.end(),
		0.0,
		[](double acc, thalweg::Location val) { return std::max(acc, val.depth); });
}

template<typename K, typename V>
auto set_contains_keys(Set<K> s, Map<K, V> m) -> bool
{
	return std::any_of(
		s.begin(),
		s.end(),
		[&](K const& key) { return m.contains(key); });
}

template<typename K, typename V>
struct DistanceComparer
{
	Map<K, V> distances;

	auto operator()(K const& lhs, K const& rhs) -> bool
	{
		if (!this->distances.contains(lhs))
			return false;
		if (!this->distances.contains(rhs))
			return true;
		return this->distances[lhs] < this->distances[rhs];
	}
};
} // namespace

Graph::Graph(std::vector<Location> data, unsigned resolution)
	: data(data)
	, resolution(resolution)
	, max_depth(max_depth_of(data))
{
}

auto Graph::contains(Coordinate coord) const -> bool
{
	return this->find(coord) != this->data.end();
}

auto Graph::adjacent(Coordinate lhs, Coordinate rhs) const -> bool
{
	return this->contains(lhs) && this->contains(rhs) && distance_between(lhs, rhs) < resolution;
}

auto Graph::weight(Coordinate coord) const -> double
{
	auto iter = this->find(coord);
	if (iter == this->data.end())
		return std::numeric_limits<double>::quiet_NaN();
	return this->max_depth - iter->depth + 1;
}

auto Graph::find(Coordinate coord) const -> DataIterator
{
	return std::find_if(
		this->data.begin(),
		this->data.end(),
		[&](Location const& loc) { return loc.coord == coord; });
}

auto Graph::shortest_path(Coordinate const& source, Coordinate const& sink) const -> std::vector<Location>
{
	auto coords = std::vector<Coordinate>{};
	std::transform(
		this->data.begin(),
		this->data.end(),
		std::back_inserter(coords),
		[](Location const& loc) { return loc.coord; });

	auto const source_on_grid = closest_point(source, coords);
	auto const sink_on_grid = closest_point(sink, coords);
	auto unvisited = Set<Coordinate>(coords.begin(), coords.end());
	auto tentative_distance = Map<Coordinate, double>();
	tentative_distance[source_on_grid] = 0.0;
	auto back_map = Map<Coordinate, Coordinate>();
	back_map[source_on_grid] = source_on_grid;

	while (unvisited.contains(sink_on_grid) && set_contains_keys(unvisited, tentative_distance))
	{
		auto current_iter = std::min_element(
			unvisited.begin(),
			unvisited.end(),
			DistanceComparer(tentative_distance));
		if (current_iter == unvisited.end())
			throw std::runtime_error("no more tentative distances");
		auto current = *current_iter;

		auto unvisited_neighbors = std::vector<Coordinate>{};
		std::copy_if(
			coords.begin(),
			coords.end(),
			std::back_inserter(unvisited_neighbors),
			[&, this](Coordinate const& coord)
			{
				return current != coord && this->adjacent(current, coord) && unvisited.contains(coord);
			});

		for (auto const& neighbor : unvisited_neighbors)
		{
			auto const iter = tentative_distance.find(current);
			auto const distance_to_here = iter == tentative_distance.end()
				? 0.0
				: iter->second;
			auto const new_distance = this->weight(neighbor) + distance_to_here;
			auto const neighbor_iter = tentative_distance.find(neighbor);
			if (neighbor_iter == tentative_distance.end() || new_distance < neighbor_iter->second)
			{
				tentative_distance[neighbor] = new_distance;
				back_map[neighbor] = current;
			}
			
		}

		unvisited.erase(current);
	}

	auto path = std::vector<Location>();
	auto current = sink_on_grid;
	while (current != source_on_grid)
	{
		path.insert(path.begin(), *this->find(current));
		current = back_map[current];
	}
	path.insert(path.begin(), *this->find(current));

	return path;
}
} // namespace thalweg
