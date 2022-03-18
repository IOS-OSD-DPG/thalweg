include(CTest)

set(TestFiles
	ParseTest.cpp
	ReadTest.cpp
	UtilsTest.cpp
	)

add_executable(thalwegTest
	Main.cpp
	${TestFiles}
	)
target_include_directories(thalwegTest PUBLIC ${DOCTEST_INCLUDE_DIR})
target_link_libraries(thalwegTest thalwegCommon)

foreach(test ${TestFiles})
	get_filename_component(TName ${test} NAME_WE)
	add_test(NAME ${TName} COMMAND thalwegTest --ts=${TName})
endforeach()
