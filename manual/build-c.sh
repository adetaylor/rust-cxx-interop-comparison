#!/bin/bash

g++ -c -std=c++14 -O3 -o c-bits.o demo.cc
ar crs libc-bits.a c-bits.o
