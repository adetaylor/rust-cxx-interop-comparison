#pragma once
#include <string>
#include <memory>
#include "rust/cxx.h"

struct Address {
    std::unique_ptr<std::string> street;
    uint64_t house_number;

    std::unique_ptr<std::string> get_street();
    uint64_t get_house_number();
};

std::unique_ptr<Address> parse_address(::rust::Str input);
