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
src/dX/dX.cpp2 src/dX/dX.h2
src/d1/d1.cpp2 src/d1/d1.h2
src/d2/d2.cpp2 src/d2/d2.h2
src/d3/d3.cpp2 src/d3/d3.h2
src/d4/d4.cpp2 src/d4/d4.h2
src/d5/d5.cpp2 src/d5/d5.h2
src/d6/d6.cpp2 src/d6/d6.h2
src/d7/d7.cpp2 src/d7/d7.h2
src/d8/d8.cpp2 src/d8/d8.h2
src/d9/d9.cpp2 src/d9/d9.h2
src/d10/d10.cpp2 src/d10/d10.h2
src/d11/d11.cpp2 src/d11/d11.h2
src/d12/d12.cpp2 src/d12/d12.h2
src/d13/d13.cpp2 src/d13/d13.h2
src/d14/d14.cpp2 src/d14/d14.h2
src/d15/d15.cpp2 src/d15/d15.h2
src/d16/d16.cpp2 src/d16/d16.h2
src/d17/d17.cpp2 src/d17/d17.h2
src/d18/d18.cpp2 src/d18/d18.h2
src/d19/d19.cpp2 src/d19/d19.h2
)