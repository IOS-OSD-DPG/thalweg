#include "Graph.hpp"
#include "Read.hpp"

#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace
{
struct CliOption
{
	char const short_name;
	std::string const long_name;
	std::string const description;

	auto matches(std::string arg) const -> bool
	{
		if (arg.front() == '-' && arg.size() == 2)
			return arg.back() == this->short_name;
		else if (arg.substr(0, 2) == "--" && arg.size() > 2)
			return arg.substr(2, long_name.size()) == this->long_name;
		else
			return false;
	}

	auto usage() const -> std::string
	{
		auto const short_name = std::string("-") + this->short_name;
		auto const long_name = "--" + this->long_name;
		return "\t" + short_name + ", " + long_name + "\t" + this->description + "\n";
	}
};

auto get_value(char const* flag, char const* extra) -> std::string
{
	auto const s = std::string(flag);
	if (size_t idx = s.find('='); idx != std::string::npos)
		return s.substr(idx + 1);
	return std::string(extra);
}

template<typename T, typename IterT, typename Fn>
auto accumulate(IterT begin, IterT end, T acc, Fn op) -> T
{
	while (begin != end)
	{
		acc = op(acc, *begin);
		++begin;
	}
	return acc;
}

auto usage(std::string const& name, std::vector<CliOption> const& options) -> std::string
{
	auto const option_description = accumulate(
		options.begin(),
		options.end(),
		std::string(""),
		[](std::string const& acc, CliOption const& option) { return acc + option.usage(); });
	return "usage: " + name + " -d <data directory> -c <corner file>\n" + "\n" + option_description;
}
} // namespace

auto main(int argc, char** argv) -> int
{
	bool help = false;
	std::string data_dir, corner_file;
	// default 10m resolution
	unsigned long resolution = 10;
	// clang-format off
	auto const help_option = CliOption {
		'h',
		"help",
		"display this help message"
	};
	auto const data_option = CliOption {
		'd',
		"data",
		"the directory containing raw data files with latitude,longitude,depth lines"
	};
	auto const corner_option = CliOption {
		'c',
		"corner",
		"the data file containing the coordinates of the corners in the inlet"
	};
	auto const resolution_option = CliOption {
		'r',
		"resolution",
		"the desired resolution (in metres) of the thalweg"
	};
	// clang-format on

	for (int i = 0; i < argc; ++i)
	{
		auto const arg = argv[i];
		auto const option = argv[i + 1];
		if (data_option.matches(arg))
		{
			data_dir = get_value(arg, option);
		}
		else if (corner_option.matches(arg))
		{
			corner_file = get_value(arg, option);
		}
		else if (resolution_option.matches(arg))
		{
			resolution = std::stoull(get_value(arg, option));
		}
		else if (help_option.matches(arg))
		{
			help = true;
		}
	}
	if (data_dir.empty() || corner_file.empty() || help)
	{
		std::cerr << usage(argv[0], {data_option, corner_option}) << std::endl;
		return 1;
	}

	auto data_path = fs::path(data_dir);
	if (!fs::exists(data_path))
	{
		std::cerr << data_dir << " does not seem to exist\n";
		return 2;
	}
	if (!fs::is_directory(data_path))
	{
		std::cerr << data_dir << " is not a directory\n";
		return 3;
	}
	std::vector<thalweg::Location> data;
	for (auto iter = fs::directory_iterator(data_path); iter != fs::directory_iterator(); ++iter)
	{
		auto const entry = *iter;
		if (!entry.exists() || !entry.is_regular_file())
			continue;
		auto const file_name = entry.path();
		if (file_name.extension() != ".txt")
			continue;
		auto data_stream = std::fstream(file_name);
		auto const tmp = thalweg::read_data(data_stream);
		data.insert(data.end(), tmp.begin(), tmp.end());
	}

	auto const corner_path = fs::path(corner_file);
	if (!fs::exists(corner_path))
	{
		std::cerr << data_dir << " does not seem to exist\n";
		return 2;
	}
	if (!fs::is_regular_file(corner_path))
	{
		std::cerr << corner_path << " is not a regular file\n";
		return 3;
	}
	auto corner_stream = std::fstream(corner_path);
	auto const corners = thalweg::read_corners(corner_stream);

	// TODO: use files in thalweg generation
	std::cout << "Read " << data.size() << " data points and " << corners.size() << " corners with resolution " << resolution << "\n";

	std::cout << "Corners indicate an inlet with the following sections:\n";
	for (size_t i = 0; i < corners.size() - 1; ++i)
	{
		auto const& start = corners[i];
		auto const& end = corners[i + 1];
		auto const distance = thalweg::distance_between(start, end);
		// clang-format off
		std::cout
			<< "Between " << start << " and " << end << " for a distance of " << (distance / 1000) << "km\n";
		// clang-format on
	}

	std::vector<thalweg::Coordinate> locations;
	std::transform(
		data.begin(),
		data.end(),
		std::back_inserter(locations),
		&thalweg::Location::coordinates);
	for (auto const& corner : corners)
	{
		auto const closest = thalweg::closest_point(corner, locations);
		std::cout << "The closest point to " << corner << " that could be found was " << closest << std::endl;
	}

	thalweg::Graph graph(data, resolution);
	std::cout << "Performing shortest path search\n";
	auto path = graph.shortest_path(corners[0], corners[1]);

	for (auto const& node : path)
	{
		std::cout << node << std::endl;
	}

	return 0;
}
