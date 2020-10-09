#include "cxx-demo/include/demo.h"
#include <sstream>
#include <iostream>

std::unique_ptr<Address> parse_address(::rust::Str input) {
    Address results;
    std::stringstream ss(std::string(to_parse));
    std::string street_address;
    ss >> results.house_number >> street_address;
    validate_house_number(results.house_number);
    results.street = std::make_unique<std::string>(street_address);
    return results;
}

uint64_t Address::get_house_number() {
    return house_number;
}

std::unique_ptr<std::string> Address::get_street() {
    return street;
}
