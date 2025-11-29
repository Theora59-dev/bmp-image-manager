#ifdef __cplusplus
extern "C" {
#endif

#include "stdint.h"

int write_bmp_from_rgba_matrix(
    const uint8_t* rgba_matrix,
    uint32_t width,
    uint32_t height,
    const char* file_path
);

#ifdef __cplusplus
}
#endif
