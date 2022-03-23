#include "Graph.hpp"

#include "Heap.hpp"

#include <algorithm>
#include <cmath>
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

template<typename K>
auto in(K const& value, Set<K> const& set) -> bool
{
	return set.count(value) > 0;
}

auto max_depth_of(std::vector<thalweg::Location> const& v) -> double
{
	return std::accumulate(
		v.begin(),
		v.end(),
		0.0,
		[](double acc, thalweg::Location val) { return std::max(acc, val.depth); });
}
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
	auto coords = std::vector<Coordinate>();
	std::transform(
		data.begin(),
		data.end(),
		std::back_inserter(coords),
		&Location::coordinates);

	auto const source_on_grid = closest_point(source, coords.begin(), coords.end());
	auto const sink_on_grid = closest_point(sink, coords.begin(), coords.end());

	auto unvisited = Set<Coordinate>(coords.begin(), coords.end());

	auto tentative_distance = Map<Coordinate, double>();
	tentative_distance[source_on_grid] = 0.0;

	auto back_map = Map<Coordinate, Coordinate>();
	back_map[source_on_grid] = source_on_grid;

	auto next_heap = PriorityHeap<Coordinate>();
	next_heap.push(source_on_grid, 0);

	while (in(sink_on_grid, unvisited) && !next_heap.empty())
	{
		Coordinate current = next_heap.pop();

		auto is_neighbor = [&, this](Coordinate const& coord)
		{
			return current != coord && this->adjacent(current, coord) && in(coord, unvisited);
		};

		for (auto const& neighbor : coords)
		{
			// avoid unnecessary copy until ranges are available
			if (!is_neighbor(neighbor))
				continue;
			auto const distance_to_here = tentative_distance[current];
			auto const new_distance = this->weight(neighbor) + distance_to_here;
			auto const neighbor_iter = tentative_distance.find(neighbor);
			if (neighbor_iter == tentative_distance.end() || new_distance < neighbor_iter->second)
			{
				tentative_distance[neighbor] = new_distance;
				back_map[neighbor] = current;

				// update queue using new distance
				auto const distance = std::lround(new_distance);
				if (neighbor_iter == tentative_distance.end())
				{
					next_heap.push(neighbor, distance);
				}
				else
				{
					next_heap.decrease_priority(neighbor, distance);
				}
			}
		}

		unvisited.erase(current);
	}

	if (in(sink_on_grid, unvisited))
		throw std::runtime_error("no path from source to sink");

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
