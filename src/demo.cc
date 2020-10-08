#include "cxx-demo/include/demo.h"
#include <sstream>

Address parse_address(::rust::Str input) {
    auto to_parse = std::string(input);
    Address results;
    std::stringstream ss(to_parse);
    std::string street_address;
    ss >> results.house_number >> street_address;
    validate_house_number(results.house_number);
    results.street = std::make_unique<std::string>(street_address);
    return results;
}
