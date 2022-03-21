include(ExternalProject)

set(DOCTEST_VERSION v2.4.8)

ExternalProject_Add(
	doctest
	PREFIX ${CMAKE_BINARY_DIR}/doctest
	URL https://github.com/doctest/doctest/archive/refs/tags/${DOCTEST_VERSION}.zip
	TIMEOUT 10
	UPDATE_COMMAND ""
	CONFIGURE_COMMAND ""
	BUILD_COMMAND ""
	INSTALL_COMMAND ""
	LOG_DOWNLOAD ON
	)

ExternalProject_Get_Property(doctest source_dir)
set(DOCTEST_INCLUDE_DIR ${source_dir}/doctest CACHE INTERNAL "Path to include folder for doctest")
