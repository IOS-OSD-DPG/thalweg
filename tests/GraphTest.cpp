#include "Graph.hpp"

#include "doctest.h"

using namespace thalweg;

// the standard forbids this, but it seems to work
namespace std
{
template<typename T>
auto operator<<(std::ostream& os, std::vector<T> const& vec) -> std::ostream&
{
	os << "[";
	for (size_t i = 0; i < vec.size(); ++i)
	{
		if (i < vec.size() - 1)
			os << vec[i] << ", ";
		else
			os << vec[i];
	}
	os << "]";
	return os;
}
} // namespace std

TEST_SUITE("GraphTest")
{
	TEST_CASE("Graph contains given data")
	{
		auto const data = std::vector<Location> {
			{Coordinate {0, 0}, 14},
		};
		auto const graph = Graph(data, 0);
		CHECK(graph.contains(Coordinate {0, 0}));
	}

	TEST_CASE("Graph only calculates distance when using points inside it")
	{
		auto const data = std::vector<Location> {};
		auto const graph = Graph(data, 200'000);
		CHECK(!graph.adjacent(Coordinate {0, 0}, Coordinate {1, 1}));
	}

	TEST_CASE("Graph determines whether points are adjacent based on resolution")
	{
		auto data = std::vector<Location> {
			{Coordinate {0, 0}, 1},
			{Coordinate {1, 1}, 1},
		};
		auto const graph1 = Graph(data, 100);
		CHECK(!graph1.adjacent(Coordinate {0, 0}, Coordinate {1, 1}));
		// (0, 0) -> (1, 1) is ~200km
		auto const graph2 = Graph(data, 200'000);
		CHECK(graph2.adjacent(Coordinate {0, 0}, Coordinate {1, 1}));
	}

	TEST_CASE("Graph determines weights based on given depths")
	{
		auto const data = std::vector<Location> {
			{Coordinate {0, 0}, 1},
			{Coordinate {1, 1}, 14},
		};
		auto const graph = Graph(data, 100);
		CHECK(graph.weight(Coordinate {0, 0}) == doctest::Approx(14));
		CHECK(graph.weight(Coordinate {1, 1}) == doctest::Approx(1));
	}

	TEST_CASE("shortest_path provides a path of nodes containing the source and sink")
	{
		auto const data = std::vector<Location> {
			{Coordinate{0, 0}, 1},
			{Coordinate{1, 1}, 1},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(Coordinate{0, 0}, Coordinate{1, 1});
		CHECK(result == data);
	}

	TEST_CASE("shortest_path provides a path that does not include useless values")
	{
		auto const data = std::vector<Location> {
			{Coordinate{0, 0}, 1},
			{Coordinate{1, 1}, 1},
			{Coordinate{10, 10}, 1},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(Coordinate{0, 0}, Coordinate{1, 1});
		CHECK(result != data);
		CHECK(result.size() == 2);
		CHECK(result[0] == Location{Coordinate{0, 0}, 1});
		CHECK(result[1] == Location{Coordinate{1, 1}, 1});
	}

	TEST_CASE("shortest_path provides a path that maps to the path of deepest values")
	{
		auto const data = std::vector<Location> {
			{Coordinate{-1, -1}, 14},
			{Coordinate{-1, 0}, 15},
			{Coordinate{-1, 1}, 10},
			{Coordinate{0, -1}, 10},
			{Coordinate{0, 0}, 9},
			{Coordinate{0, 1}, 14},
			{Coordinate{1, -1}, 5},
			{Coordinate{1, 0}, 6},
			{Coordinate{1, 1}, 10},
		};
		auto const expected = std::vector<Location> {
			{Coordinate{-1, -1}, 14},
			{Coordinate{-1, 0}, 15},
			{Coordinate{0, 1}, 14},
			{Coordinate{1, 1}, 10},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(Coordinate{-1, -1}, Coordinate{1, 1});
		CHECK(result == expected);
	}
}
