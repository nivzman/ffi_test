#include "library.h"

#include <stdio.h>
#include <stdlib.h>

size_t square(size_t a) {
    return a*a;
}

void output_param(size_t in, size_t* out) {
    *out = in * 10;
}

int fill(const char* input_buff, size_t input_size, char* output_buff, size_t output_size) {
    if (input_size == 0) {
        return -1;
    }

    size_t index = 0;
    int full = 0;
    for (size_t i = 0; i<output_size; i++) {
        output_buff[i] = input_buff[index];
        index++;

        if (index >= input_size) {
            index = 0;
            full++;
        }
    }
    return full;
}


struct MyS {
    int a;
    char b;
} my_struct;


struct MyS get_struct() {
    struct MyS s;
    s.a = 5;
    s.b = 'w';
    return s;
}

struct MyS* get_struct_ptr() {
    struct MyS* s_ptr = (struct MyS*)(malloc(sizeof(struct MyS)));
    s_ptr->a = 7;
    s_ptr->b = 'l';
    return s_ptr;
}


