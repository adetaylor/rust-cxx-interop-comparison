#!/bin/bash

g++ -c -o c-bits.o demo.cc
ar crs libc-bits.a c-bits.o
