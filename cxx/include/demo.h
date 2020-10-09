#pragma once
#include "rust/cxx.h"
#include "cxx-demo/src/main.rs.h"

Address parse_address(::rust::Str input);
