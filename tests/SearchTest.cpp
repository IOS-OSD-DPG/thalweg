#include "Search.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("SearchTest")
{
	TEST_CASE("sparse neighborhood")
	{
		CoordinatePair const location = CoordinatePair{Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)};

		auto const tree = SearchTree({location});
		auto const neighborhood = tree.neighbors(location);

		CHECK(neighborhood.size() == 1);
		CHECK(neighborhood.count(location) == 1);
	}

	TEST_CASE("single neighborhood")
	{
		CoordinatePair point = {Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)};

		auto const data = std::vector<CoordinatePair> {
			{Latitude({0, 0, 0.2}, true), Longitude({0, 0, 0.1}, true)},
			{Latitude({0, 0, 0.5}, true), Longitude({0, 0, 0.2}, true)},
			{Latitude({0, 0, 0.7}, true), Longitude({0, 0, 0.3}, true)},
			{Latitude({0, 0, 0.9}, true), Longitude({0, 0, 0.4}, true)},
			{Latitude({0, 0, 0.3}, true), Longitude({0, 0, 0.5}, true)},
			{Latitude({0, 0, 0.6}, true), Longitude({0, 0, 0.6}, true)},
		};

		auto const tree = SearchTree(data);
		auto const neighborhood = tree.neighbors(point);

		CHECK(neighborhood.size() == data.size());
	}

	TEST_CASE("multiple neighborhoods")
	{
		CoordinatePair point = {Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)};

		auto const data = std::vector<CoordinatePair> {
			{Latitude({0, 0, 0.2}, true), Longitude({0, 0, 0.1}, true)},
			{Latitude({0, 0, 0.5}, true), Longitude({0, 0, 0.2}, true)},
			{Latitude({0, 0, 0.7}, false), Longitude({0, 0, 0.3}, true)},
			{Latitude({0, 0, 0.9}, true), Longitude({0, 0, 0.4}, true)},
			{Latitude({0, 0, 0.3}, false), Longitude({0, 0, 0.5}, false)},
			{Latitude({0, 0, 0.6}, true), Longitude({0, 0, 0.6}, false)},
		};

		auto const tree = SearchTree(data);
		auto const neighborhood = tree.neighbors(point);

		CHECK(neighborhood.size() == data.size());
	}

	TEST_CASE("excludes values too far away")
	{
		CoordinatePair point = {Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)};

		auto const data = std::vector<CoordinatePair> {
			{Latitude({0, 0, 0.2}, true), Longitude({0, 0, 0.1}, true)},
			{Latitude({0, 0, 0.5}, true), Longitude({0, 0, 0.2}, true)},
			{Latitude({0, 0, 0.7}, false), Longitude({0, 0, 0.3}, true)},
			{Latitude({0, 0, 0.9}, true), Longitude({0, 0, 0.4}, true)},
			{Latitude({0, 0, 0.3}, false), Longitude({0, 0, 0.5}, false)},
			{Latitude({1, 0, 0.6}, true), Longitude({0, 0, 0.6}, false)},
		};

		auto const tree = SearchTree(data);
		auto const neighborhood = tree.neighbors(point);

		CHECK(neighborhood.size() == data.size() - 1);
	}

	TEST_CASE("closest_point for point in bucket")
	{
		auto const data = std::vector<CoordinatePair> {
			{Latitude({0, 0, 0.2}, true), Longitude({0, 0, 0.1}, true)},
			{Latitude({0, 0, 0.5}, true), Longitude({0, 0, 0.2}, true)},
			{Latitude({0, 0, 0.7}, false), Longitude({0, 0, 0.3}, true)},
			{Latitude({0, 0, 0.9}, true), Longitude({0, 0, 0.4}, true)},
			{Latitude({0, 0, 0.3}, false), Longitude({0, 0, 0.5}, false)},
			{Latitude({1, 0, 0.6}, true), Longitude({0, 0, 0.6}, false)},
		};

		auto const tree = SearchTree(data);
		
		CHECK(tree.closest_point(data[0]) == data[0]);
		CHECK(tree.closest_point({Latitude({0, 0, 0}, true), Longitude({0, 0, 0}, true)}) == data[0]);
	}

	TEST_CASE("closest_point for point with no bucket")
	{
		auto const data = std::vector<CoordinatePair> {
			{Latitude({0, 0, 0.2}, true), Longitude({0, 0, 0.1}, true)},
		};

		auto const tree = SearchTree(data);
		
		CHECK(tree.closest_point({Latitude({1, 0, 0}, true), Longitude({1, 0, 0}, true)}) == data[0]);
	}
}
