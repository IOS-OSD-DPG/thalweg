#include "Graph.hpp"

#include "DisplayVector.hpp"

#include "doctest.h"

using namespace thalweg;

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

	TEST_CASE("shortest_path provides a path that roughly maps to the path of deepest values")
	{
		// need an order of magnitude difference between the deepest point and the closer ones
		auto const km = 1000.0;
		auto const data = std::vector<Location> {
			{Coordinate{-1, -1}, 140 * km},
			{Coordinate{-1, 0}, 150 * km},
			{Coordinate{-1, 1}, 100 * km},
			{Coordinate{0, -1}, 100 * km},
			{Coordinate{0, 0}, 9 * km},
			{Coordinate{0, 1}, 140 * km},
			{Coordinate{1, -1}, 5 * km},
			{Coordinate{1, 0}, 6 * km},
			{Coordinate{1, 1}, 100 * km},
		};
		auto const expected = std::vector<Location> {
			{Coordinate{-1, -1}, 140 * km},
			{Coordinate{-1, 0}, 150 * km},
			{Coordinate{0, 1}, 140 * km},
			{Coordinate{1, 1}, 100 * km},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(Coordinate{-1, -1}, Coordinate{1, 1});
		CHECK(result == expected);
	}
}
