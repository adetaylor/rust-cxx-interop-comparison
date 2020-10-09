#include <stdint.h>
#include <stddef.h>

struct MiniString {
    const char* data;
    size_t len;
};

struct Address {
    struct MiniString street;
    uint64_t house_number;
};

extern "C" void free_mini_string(struct MiniString* str);

extern "C" struct Address parse_address(const struct MiniString* unparsed_address);

