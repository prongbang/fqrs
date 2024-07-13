#ifndef libfqrs_h
#define libfqrs_h

#include <stdint.h>

int32_t decode_qr_code(const uint8_t* data, int32_t width, int32_t height, char* out_buffer, int32_t out_size);

#endif
