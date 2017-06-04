cmake_minimum_required (VERSION 3.8)
project (DNSTunnel C CXX)

# Version Number
set (Tutorial_VERSION_MAJOR 0)
set (Tutorial_VERSION_MINOR 1)

# Set output directories
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Sources
SET (SERVER_SOURCES src/main/common/constants.h
		    src/main/server/main.cpp
    		    src/main/server/server.h
    		    src/main/server/udp_server.h
    		    src/main/server/udp_server.cpp)

# Target Definitions
add_executable(server ${SERVER_SOURCES})
target_compile_options(server PUBLIC -std=c++1y -Wall -Wfloat-conversion)
target_include_directories(server PUBLIC src/main/server)

# Dependencies
find_package(Boost 1.64.0 REQUIRED filesystem system)
target_link_libraries(server PUBLIC
  ${Boost_LIBRARIES}
)

# Testing
find_package(GTest)
if(GTEST_FOUND)
  # TODO!!!

  #add_executable(unit_tests ${sources_test} ${sources})

  # we add this define to prevent collision with the main
  # this might be better solved by not adding the source with the main to the
  # testing target
  #target_compile_definitions(unit_tests PUBLIC UNIT_TESTS)

  # this allows us to use our executable as a link library
  # therefore we can inherit all compiler options and library dependencies
  #set_target_properties(example PROPERTIES ENABLE_EXPORTS on)

  #target_link_libraries(unit_tests PUBLIC
  #  ${GTEST_BOTH_LIBRARIES}
  #  example
  #)

  #target_include_directories(unit_tests PUBLIC
  #  ${GTEST_INCLUDE_DIRS} # doesn't do anything on Linux
  #)
endif()
