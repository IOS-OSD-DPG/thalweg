set(SourceFiles
	Read.cpp
	Parse.cpp
	Utils.cpp
	Coordinate.cpp
	Location.cpp
	Graph.cpp
	Search.cpp
	)

add_library(thalwegCommon STATIC
	${SourceFiles}
	)
target_include_directories(thalwegCommon PUBLIC ../include)

add_executable(thalweg
	Main.cpp
	)
target_link_libraries(thalweg thalwegCommon)
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
	if(CMAKE_CXX_COMPILER_VERSION VERSION_LESS "9.0")
		target_link_libraries(thalweg c++fs)
	endif()
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
	if(CMAKE_CXX_COMPILER_VERSION VERSION_LESS "9.1")
		target_link_libraries(thalweg stdc++fs)
	endif()
endif()
