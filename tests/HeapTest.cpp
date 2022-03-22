#include "Heap.hpp"

#include "doctest.h"

using namespace thalweg;

TEST_SUITE("HeapTest")
{
	TEST_CASE("MaxHeap begins empty")
	{
		auto heap = MaxHeap<int>();
		CHECK(heap.empty());
	}

	TEST_CASE("MaxHeap contains given elements")
	{
		auto heap = MaxHeap<int>();
		heap.push(1);
		CHECK(heap.pop() == 1);
	}

	TEST_CASE("MaxHeap returns values in order")
	{
		auto heap = MaxHeap<int>();
		heap.push(1);
		heap.push(3);
		heap.push(2);
		CHECK(heap.pop() == 3);
		CHECK(heap.pop() == 2);
		CHECK(heap.pop() == 1);
	}

	TEST_CASE("MaxHeap becomes empty after last pop")
	{
		auto heap = MaxHeap<int>();
		heap.push(1);
		CHECK(!heap.empty());
		CHECK(heap.pop() == 1);
		CHECK(heap.empty());
	}

	TEST_CASE("MaxHeap can use given function")
	{
		auto heap = MaxHeap<std::vector<int>>(
			[](std::vector<int> const& lhs, std::vector<int> const& rhs)
			{
				return lhs.size() < rhs.size();
			});
		heap.push({1});
		heap.push({});
		heap.push({1, 2});
		CHECK(heap.pop() == std::vector<int>{1, 2});
		CHECK(heap.pop() == std::vector<int>{1});
		CHECK(heap.pop() == std::vector<int>{});
	}

	TEST_CASE("MinHeap starts empty")
	{
		auto heap = MinHeap<int>();
		CHECK(heap.empty());
	}

	TEST_CASE("MinHeap contains given elements")
	{
		auto heap = MinHeap<int>();
		heap.push(1);
		CHECK(heap.pop() == 1);
	}

	TEST_CASE("MinHeap returns values in ascending order")
	{
		auto heap = MinHeap<int>();
		heap.push(3);
		heap.push(1);
		heap.push(2);
		CHECK(heap.pop() == 1);
		CHECK(heap.pop() == 2);
		CHECK(heap.pop() == 3);
	}

	TEST_CASE("MinHeap can use given function")
	{
		auto heap = MinHeap<std::vector<int>>(
			[](std::vector<int> const& lhs, std::vector<int> const& rhs)
			{
				return lhs.size() < rhs.size();
			});
		heap.push({1});
		heap.push({});
		heap.push({1, 2});
		CHECK(heap.pop() == std::vector<int>{});
		CHECK(heap.pop() == std::vector<int>{1});
		CHECK(heap.pop() == std::vector<int>{1, 2});
	}

	TEST_CASE("MinHeap becomes empty after last pop")
	{
		auto heap = MinHeap<int>();
		heap.push(1);
		CHECK(!heap.empty());
		CHECK(heap.pop() == 1);
		CHECK(heap.empty());
	}

	TEST_CASE("PriorityHeap provides values according to their priority")
	{
		auto heap = PriorityHeap<int>();
		heap.push(1, 10);
		heap.push(2, 5);
		CHECK(heap.pop() == 2);
		CHECK(heap.pop() == 1);
	}

	TEST_CASE("PriorityHeap can decrease priority")
	{
		auto heap = PriorityHeap<int>();
		heap.push(1, 10);
		heap.push(2, 5);
		heap.decrease_priority(1, 6);
		CHECK(heap.pop() == 1);
		CHECK(heap.pop() == 2);
	}
}
