#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>


template<typename T>
struct Vec;


extern "C" {

Vec<float> bootstrap_rs(const Vec<float> *x, const int32_t *_r);

} // extern "C"
