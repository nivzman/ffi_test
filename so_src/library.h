#ifndef DAMM_LIBRARY_H
#define DAMM_LIBRARY_H

#include <stddef.h>

size_t square(size_t a);

void output_param(size_t in, size_t* out);

int fill(const char* input_buff, size_t input_size, char* output_buff, size_t output_size);

#endif //DAMM_LIBRARY_H
