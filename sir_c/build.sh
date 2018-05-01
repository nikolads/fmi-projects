#!/bin/bash

# too lazy to use CMake
c++ --std=c++11 -Wall -pthreads -I./src src/main.cpp src/sir/model.cpp src/sir/simul_result.cpp src/fit/grad_desc.cpp src/fit/simul_annealing.cpp -o main
