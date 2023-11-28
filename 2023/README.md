# cppfront playground

This is a small sample project that bootstraps [cppfront](https://github.com/hsutter/cppfront) and provides a playground for running `cpp2` code.

## Building

Requires CMake 3.23+ and a C++20 compiler.

- Clone repo
- Initialize submodules (`git submodule update --init --recursive`)

### VS Code
- Open workspace in VS Code
- Install suggested extensions
- Run `CMake: Configure` command from the command palette to pick which CMake kit to use (CTRL+P -> `Cmake: Configure`)
- Select "C++ Launch" in the "Run and Debug" tab and run the `"main"` project

### Visual Studio
- Create `/build` (`mkdir build`)
- Run `cmake ..` from `/build` (`cmake ../ -G "Visual Studio 17 2022"`)
- Open `cpp2_sandbox.sln`
- Set `main` as the startup project in the Solution Explorer
- Build and run!

## Structure
cpp2 code goes in `src`

The project is currently hardcoded to only include `main.cpp2`.
