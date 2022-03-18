set(SourceFiles
	Read.cpp
	Parse.cpp
	Utils.cpp
	Coordinate.cpp
	Location.cpp
	)

add_library(thalwegCommon STATIC
	${SourceFiles}
	)
target_include_directories(thalwegCommon PUBLIC ../include)

add_executable(thalweg
	Main.cpp
	)
target_link_libraries(thalweg thalwegCommon)
