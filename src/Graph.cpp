#include "Graph.hpp"

#include "Heap.hpp"

#include <algorithm>
#include <cmath>
#include <numeric>
#include <unordered_map>
#include <unordered_set>

namespace thalweg
{
namespace
{
class ShortestPathState
{
public:
	ShortestPathState(CoordinatePair source, CoordinatePair sink)
		: visited_set()
		, sink(sink)
	{
		update(source, 0.0, source);
	}

	auto update(CoordinatePair destination, double distance, CoordinatePair previous) -> void
	{
		bool const already_present = contains(destination);
		auto const priority = std::lround(distance + distance_between(sink, destination));
		state[destination] = std::make_pair(distance, previous);
		if (already_present)
		{
			work_queue.decrease_priority(destination, priority);
		}
		else
		{
			work_queue.push(destination, priority);
		}
	}

	auto distance_to(CoordinatePair const& destination) const -> double
	{
		return state.at(destination).first;
	}

	auto previous(CoordinatePair const& destination) const -> CoordinatePair
	{
		return state.at(destination).second;
	}

	auto contains(CoordinatePair const& destination) const -> bool
	{
		return state.find(destination) != state.end();
	}

	auto get_next() -> CoordinatePair
	{
		return work_queue.pop();
	}

	auto work_remains() -> bool
	{
		return !work_queue.empty();
	}

	auto unvisited(CoordinatePair const& destination) -> bool
	{
		return visited_set.count(destination) == 0;
	}

	auto visit(CoordinatePair const& destination) -> void
	{
		visited_set.insert(destination);
	}

private:
	std::unordered_set<CoordinatePair> visited_set;
	CoordinatePair sink;
	std::unordered_map<CoordinatePair, std::pair<double, CoordinatePair>> state;
	PriorityHeap<CoordinatePair> work_queue;
};
} // namespace

Graph::Graph(std::vector<Location> data, unsigned resolution)
	: data(data)
	, search_tree(to_coordinates(data))
	, resolution(resolution)
	, max_depth(max_depth_of(data))
{
}

auto Graph::contains(CoordinatePair coord) const -> bool
{
	return this->find(coord) != this->data.end();
}

auto Graph::adjacent(CoordinatePair lhs, CoordinatePair rhs) const -> bool
{
	return this->contains(lhs) && this->contains(rhs) && distance_between(lhs, rhs) < resolution;
}

auto Graph::weight(CoordinatePair coord) const -> double
{
	auto iter = this->find(coord);
	if (iter == this->data.end())
		return std::numeric_limits<double>::infinity();
	return this->max_depth - iter->depth + 1;
}

auto Graph::find(CoordinatePair coord) const -> DataIterator
{
	return std::find_if(
		this->data.begin(),
		this->data.end(),
		[&](Location const& loc) { return loc.coord == coord; });
}

auto Graph::shortest_path(CoordinatePair const& source, CoordinatePair const& sink) const -> std::vector<Location>
{
	auto const source_on_grid = search_tree.closest_point(source);
	auto const sink_on_grid = search_tree.closest_point(sink);

	auto state = ShortestPathState(source_on_grid, sink_on_grid);

	while (state.unvisited(sink_on_grid) && state.work_remains())
	{
		CoordinatePair current = state.get_next();

		auto is_neighbor = [&, this](CoordinatePair const& coord)
		{
			return current != coord && this->adjacent(current, coord) && state.unvisited(coord);
		};

		auto const neighbors = search_tree.neighbors(current);

		for (auto const& neighbor : neighbors)
		{
			// avoid unnecessary copy until ranges are available
			if (!is_neighbor(neighbor))
				continue;
			auto const distance_to_here = state.distance_to(current);
			auto const new_distance = this->weight(neighbor) + distance_to_here;
			if (!state.contains(neighbor) || new_distance < state.distance_to(neighbor))
			{
				state.update(neighbor, new_distance, current);
			}
		}

		state.visit(current);
	}

	if (state.unvisited(sink_on_grid))
		throw std::runtime_error("no path from source to sink");

	auto path = std::vector<Location>();
	auto current = sink_on_grid;
	while (current != source_on_grid)
	{
		path.push_back(*this->find(current));
		current = state.previous(current);
	}
	path.push_back(*this->find(current));

	std::reverse(path.begin(), path.end());
	return path;
}
} // namespace thalweg
