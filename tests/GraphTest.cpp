#include "Graph.hpp"

#include "DisplayVector.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("GraphTest")
{
	TEST_CASE("Graph contains given data")
	{
		auto const data = std::vector<Location> {
			{CoordinatePair {0, 0}, 14},
		};
		auto const graph = Graph(data, 0);
		CHECK(graph.contains(CoordinatePair {0, 0}));
	}

	TEST_CASE("Graph only calculates distance when using points inside it")
	{
		auto const data = std::vector<Location> {};
		auto const graph = Graph(data, 200'000);
		CHECK(!graph.adjacent(CoordinatePair {0, 0}, CoordinatePair {1, 1}));
	}

	TEST_CASE("Graph determines whether points are adjacent based on resolution")
	{
		auto data = std::vector<Location> {
			{CoordinatePair {0, 0}, 1},
			{CoordinatePair {1, 1}, 1},
		};
		auto const graph1 = Graph(data, 100);
		CHECK(!graph1.adjacent(CoordinatePair {0, 0}, CoordinatePair {1, 1}));
		// (0, 0) -> (1, 1) is ~200km
		auto const graph2 = Graph(data, 200'000);
		CHECK(graph2.adjacent(CoordinatePair {0, 0}, CoordinatePair {1, 1}));
	}

	TEST_CASE("Graph determines weights based on given depths")
	{
		auto const data = std::vector<Location> {
			{CoordinatePair {0, 0}, 1},
			{CoordinatePair {1, 1}, 14},
		};
		auto const graph = Graph(data, 100);
		CHECK(graph.weight(CoordinatePair {0, 0}) == doctest::Approx(14));
		CHECK(graph.weight(CoordinatePair {1, 1}) == doctest::Approx(1));
	}

	TEST_CASE("shortest_path provides a path of nodes containing the source and sink")
	{
		auto const data = std::vector<Location> {
			{CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)}, 1},
			{CoordinatePair{Latitude({0, 0, 0.1}, true), Longitude({0, 0, 0.1}, true)}, 1},
		};
		auto graph = Graph(data, 400);
		auto const result = graph.shortest_path(data.front().coord, data.back().coord);
		CHECK(result == data);
	}

	TEST_CASE("shortest_path provides a path that does not include useless values")
	{
		auto const data = std::vector<Location> {
			{CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)}, 1},
			{CoordinatePair{Latitude({0, 0, 0.1}, true), Longitude({0, 0, 0.1}, true)}, 1},
			{CoordinatePair{Latitude({0, 0, 1}, true), Longitude({0, 0, 1}, true)}, 1},
			{CoordinatePair{Latitude({0, 0, 1.1}, true), Longitude({0, 0, 1.1}, true)}, 1},
		};
		auto graph = Graph(data, 400);
		auto const result = graph.shortest_path(data[0].coord, data[1].coord);
		CHECK(result != data);
		CHECK(result.size() == 2);
		CHECK(result[0] == data[0]);
		CHECK(result[1] == data[1]);
	}

	TEST_CASE("shortest_path provides a path that roughly maps to the path of deepest values")
	{
		// need an order of magnitude difference between the deepest point and the closer ones
		auto const km = 1000.0;
		auto const data = std::vector<Location> {
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 1}, false)}, 140 * km},
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 0}, true)}, 150 * km},
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 1}, true)}, 100 * km},
			{CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 1}, false)}, 100 * km},
			{CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)}, 9 * km},
			{CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 1}, true)}, 140 * km},
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 1}, false)}, 5 * km},
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 0}, true)}, 6 * km},
			{CoordinatePair{Latitude({0, 0, 1}, false), Longitude({0, 0, 1}, true)}, 100 * km},
		};
		// inclusion of search tree has resulted in skipping the second 140km depth
		auto const expected = std::vector<Location> {data[0], data[1], data[8]};
		auto graph = Graph(data, 50);
		auto const result = graph.shortest_path(data.front().coord, data.back().coord);
		CHECK(result == expected);
	}
}
