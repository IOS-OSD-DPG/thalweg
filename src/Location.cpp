#include "Location.hpp"

namespace thalweg
{
auto Location::coordinates(Location const& loc) -> Coordinate
{
	return loc.coord;
}

auto operator==(Location const& lhs, Location const& rhs) -> bool
{
	return lhs.coord == rhs.coord && lhs.depth == rhs.depth;
}

auto operator<<(std::ostream& os, Location const& value) -> std::ostream&
{
	os << "Location:{coord:" << value.coord << ", depth:" << value.depth << "}";
	return os;
}
} // namespace thalweg
