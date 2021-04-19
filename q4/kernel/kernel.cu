#include <stdio.h>

// Use this as the random seed in the pseudorandom function (see instructions)
#define SEED 20210418

// Pseudo-random number generator. This provides consistent reproducible results so that
// you can test your program.
__device__ float pseudorandom( uint x ) {
    uint value = x;
    value = (value ^ 61) ^ (value>>16);
    value *= 9;
    value ^= value << 4;
    value *= 0x27d4eb2d;
    value ^= value >> 15;
    return (float) value / (float) INT_MAX;
}
