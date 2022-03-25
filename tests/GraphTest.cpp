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
			{CoordinatePair{0, 0}, 1},
			{CoordinatePair{1, 1}, 1},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(CoordinatePair{0, 0}, CoordinatePair{1, 1});
		CHECK(result == data);
	}

	TEST_CASE("shortest_path provides a path that does not include useless values")
	{
		auto const data = std::vector<Location> {
			{CoordinatePair{0, 0}, 1},
			{CoordinatePair{1, 1}, 1},
			{CoordinatePair{10, 10}, 1},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(CoordinatePair{0, 0}, CoordinatePair{1, 1});
		CHECK(result != data);
		CHECK(result.size() == 2);
		CHECK(result[0] == Location{CoordinatePair{0, 0}, 1});
		CHECK(result[1] == Location{CoordinatePair{1, 1}, 1});
	}

	TEST_CASE("shortest_path provides a path that roughly maps to the path of deepest values")
	{
		// need an order of magnitude difference between the deepest point and the closer ones
		auto const km = 1000.0;
		auto const data = std::vector<Location> {
			{CoordinatePair{-1, -1}, 140 * km},
			{CoordinatePair{-1, 0}, 150 * km},
			{CoordinatePair{-1, 1}, 100 * km},
			{CoordinatePair{0, -1}, 100 * km},
			{CoordinatePair{0, 0}, 9 * km},
			{CoordinatePair{0, 1}, 140 * km},
			{CoordinatePair{1, -1}, 5 * km},
			{CoordinatePair{1, 0}, 6 * km},
			{CoordinatePair{1, 1}, 100 * km},
		};
		auto const expected = std::vector<Location> {
			{CoordinatePair{-1, -1}, 140 * km},
			{CoordinatePair{-1, 0}, 150 * km},
			{CoordinatePair{0, 1}, 140 * km},
			{CoordinatePair{1, 1}, 100 * km},
		};
		auto graph = Graph(data, 200'000);
		auto const result = graph.shortest_path(CoordinatePair{-1, -1}, CoordinatePair{1, 1});
		CHECK(result == expected);
	}
}
