#include <stdint.h>
#include <stdio.h>

int main() {
    // The highest 8-bit unsigned integer possible is 255,
    // or 2^8 - 1, the highest number you can represent
    // before you need a 2^8 or 256 place in binary.
    uint8_t integer = 255;

    // OK, so what happens if we add 1 to it?
    integer += 1;

    // Let's print it out and see!
    printf("%u\n", integer);
}
