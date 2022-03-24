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
	ShortestPathState(std::vector<Coordinate> container, Coordinate source, Coordinate sink)
		: unvisited_set(std::begin(container), std::end(container))
		, sink(sink)
	{
		update(source, 0.0, source);
	}

	auto update(Coordinate destination, double distance, Coordinate previous) -> void
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

	auto distance_to(Coordinate const& destination) const -> double
	{
		return state.at(destination).first;
	}

	auto previous(Coordinate const& destination) const -> Coordinate
	{
		return state.at(destination).second;
	}

	auto contains(Coordinate const& destination) const -> bool
	{
		return state.find(destination) != state.end();
	}

	auto get_next() -> Coordinate
	{
		return work_queue.pop();
	}

	auto work_remains() -> bool
	{
		return !work_queue.empty();
	}

	auto unvisited(Coordinate const& destination) -> bool
	{
		return unvisited_set.count(destination) > 0;
	}

	auto visit(Coordinate const& destination) -> void
	{
		unvisited_set.erase(destination);
	}

private:
	std::unordered_set<Coordinate> unvisited_set;
	Coordinate sink;
	std::unordered_map<Coordinate, std::pair<double, Coordinate>> state;
	PriorityHeap<Coordinate> work_queue;
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
	auto const coords = to_coordinates(this->data);

	auto const source_on_grid = closest_point(source, coords);
	auto const sink_on_grid = closest_point(sink, coords);

	auto state = ShortestPathState(coords, source_on_grid, sink_on_grid);

	while (state.unvisited(sink_on_grid) && state.work_remains())
	{
		Coordinate current = state.get_next();

		auto is_neighbor = [&, this](Coordinate const& coord)
		{
			return current != coord && this->adjacent(current, coord) && state.unvisited(coord);
		};

		for (auto const& neighbor : coords)
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
