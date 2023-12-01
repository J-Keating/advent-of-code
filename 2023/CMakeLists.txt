cmake_minimum_required(VERSION 3.23)
project(cpp2_sandbox)

include(FetchContent)

add_subdirectory(src_deps/cppfront_cmake)

# cppfront flags
# -p - "Pure" cpp2, disable cpp1 mixing
# -format-colon-errors - Emit ':line:col:' format for messages 
set(CPPFRONT_FLAGS "-p" "-format-colon-errors")

add_executable(main src/main.cpp2
src/test/test.cpp2 src/test/test.h2
src/d1_2022/d1_2022.cpp2 src/d1_2022/d1_2022.h2
src/d1/d1.cpp2 src/d1/d1.h2
)