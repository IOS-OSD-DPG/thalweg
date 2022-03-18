#pragma once

#include <string>

namespace thalweg
{
	auto parse_dms_latitude(std::string const&) -> double;
	auto parse_dms_longitude(std::string const&) -> double;
	auto parse_depth(std::string const&) -> double;
}
