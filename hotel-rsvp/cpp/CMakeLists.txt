project(HotelRsvo)
cmake_minimum_required(VERSION 2.8.12)

enable_testing()
find_package(GTest CONFIG REQUIRED)
find_package(Threads REQUIRED)

add_definitions(-std=c++17)
set(CMAKE_CXX_FLAGS "${CMAXE_CXX_FLAGS} -Wall")


set(sources
    Client.h
    Client.cpp
    Hotel.h
    Hotel.cpp
    Model.h 
    Model.cpp)

set(testSources
    HotelTest.cpp
    ModelTest.cpp
    ClientTest.cpp)

add_executable(test ${testSources} ${sources})
target_link_libraries(test  PRIVATE Threads::Threads GTest::gmock GTest::gtest GTest::gmock_main GTest::gtest_main)
add_test(test-1 test)
