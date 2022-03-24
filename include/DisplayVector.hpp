#pragma once

#include <iostream>
#include <vector>

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
