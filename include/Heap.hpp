#pragma once

#include <execution>
#include <functional>
#include <vector>
#include <utility>

namespace thalweg
{
template<typename T>
using CompareFn = std::function<bool(T const&, T const&)>;

template<typename T>
CompareFn<T> negate(CompareFn<T> fn)
{
	return [=](T const& lhs, T const& rhs){ return !fn(lhs, rhs); };
}

template<typename T>
class MaxHeap
{
public:
	MaxHeap() : comp([](T const& lhs, T const& rhs){ return lhs < rhs; })
	{}

	explicit MaxHeap(CompareFn<T> comp) : comp(comp)
	{}

	auto push(T value) -> void
	{
		auto iter = std::find(data.begin(), data.end(), value);
		if (iter != data.end())
			return;
		data.push_back(value);
		std::push_heap(data.begin(), data.end(), comp);
	}

	auto pop() -> T
	{
		std::pop_heap(data.begin(), data.end(), comp);
		auto value = data.back();
		data.pop_back();
		return value;
	}

	auto empty() -> bool
	{
		return data.empty();
	}

protected:
	std::vector<T> data;
	CompareFn<T> comp;
};

template<typename T>
class MinHeap : public MaxHeap<T>
{
public:
	MinHeap() : MaxHeap<T>([](T const& lhs, T const& rhs){ return lhs > rhs; })
	{}

	explicit MinHeap(CompareFn<T> comp) : MaxHeap<T>(negate(comp))
	{}
};

template<typename T>
class PriorityHeap : public MinHeap<std::pair<long, T>>
{
public:
	PriorityHeap() : MinHeap<std::pair<long, T>>([](auto const& lhs, auto const& rhs){ return lhs.first < rhs.first; })
	{}

	auto push(T value, long priority) -> void
	{
		MaxHeap<std::pair<long, T>>::push(std::make_pair(priority, value));
	}

	auto pop() -> T
	{
		return MaxHeap<std::pair<long, T>>::pop().second;
	}

	auto decrease_priority(T value, long priority) -> void
	{
		auto const iter = std::find_if(
			std::execution::par_unseq,
			this->data.begin(),
			this->data.end(),
			[&](auto elem){ return elem.second == value; });
		if (iter != this->data.end())
		{
			iter->first = priority;
			std::make_heap(this->data.begin(), this->data.end(), this->comp);
		}
	}
};
} // namespace thalweg
