#include "demo.hpp"
#include <sstream>
#include <string>

void free_mini_string(struct MiniString* str) {
    delete[] str->data;
    str->data = nullptr;
}

extern "C" void validate_house_number(uint64_t number);

struct Address parse_address(const struct MiniString* str) {
    auto to_parse = std::string(str->data, str->len);
    struct Address results;
    std::stringstream ss(to_parse);
    std::string street_address;
    ss >> results.house_number >> street_address;
    validate_house_number(results.house_number);
    results.street.len = street_address.size();
    char* streetstr = new char[results.street.len];
    memcpy(streetstr, street_address.c_str(), results.street.len);
    results.street.data = streetstr;
    return results;
}
