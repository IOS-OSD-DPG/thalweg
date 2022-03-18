#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

#include "Read.hpp"

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
				return arg.substr(2) == this->long_name;
			else
				return false;
		}

		auto usage() const -> std::string
		{
			auto short_name = std::string("-") + this->short_name;
			auto long_name = "--" + this->long_name;
			return "\t" + short_name + ", " + long_name + "\t\t" + this->description + "\n";
		}
	};

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
		auto option_description = accumulate(
			options.begin(),
			options.end(),
			std::string(""),
			[](std::string const& acc, CliOption const& option) { return acc + option.usage(); });
		return "usage: " + name + " -d <data file> -c <corner file>\n"
			+ "\n"
			+ option_description
			;
	}
}

auto main(int argc, char** argv) -> int
{
	bool help = false;
	std::string data_file, corner_file;
	auto help_option = CliOption {
		'h',
		"help",
		"display this help message"
	};
	auto data_option = CliOption {
		'd',
		"data",
		"the raw data file contianing latitude,longitude,depth lines"
	};
	auto corner_option = CliOption {
		'c',
		"corner",
		"the data file containing the coordinates of the corners in the inlet"
	};

	for (int i = 0; i < argc; ++i)
	{
		auto arg = argv[i];
		if (data_option.matches(arg))
		{
			data_file = argv[i+1];
		}
		else if (corner_option.matches(arg))
		{
			corner_file = argv[i+1];
		}
		else if (help_option.matches(arg))
		{
			help = true;
		}
	}
	if (data_file.empty() || corner_file.empty() || help)
	{
		std::cout << usage(argv[0], {data_option, corner_option}) << std::endl;
		return 1;
	}

	auto data_path = fs::path(data_file);
	if (!fs::exists(data_path))
	{
		std::cout << data_file << " does not seem to exist\n";
		return 2;
	}
	//auto data = thalweg::read_data(data_path);

	auto corner_path = fs::path(corner_file);
	if (!fs::exists(corner_path))
	{
		std::cout << data_file << " does not seem to exist\n";
		return 2;
	}
	//auto corners = thalweg::read_corners(corner_path);
	return 0;
}
