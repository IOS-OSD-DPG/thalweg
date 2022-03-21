#pragma once

#include <string>
#include <vector>

namespace thalweg
{
namespace utils
{
auto split(std::string const&, char) -> std::vector<std::string>;
}
} // namespace thalweg
