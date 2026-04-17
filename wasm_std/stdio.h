#pragma once

#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned long size_t;

extern void* stdout;
extern void* stderr;

void printf(const char* fmt, ...);

int fflush(void* stream);

int vsprintf_s(char* buffer, size_t sizeOfBuffer, const char* format, va_list argptr);

int sprintf_s(char* buffer, size_t sizeOfBuffer, const char* format, ...);

int vsnprintf(char* str, size_t size, const char* format, va_list arg);

int snprintf(char* str, size_t size, const char* format, ...);

#ifdef __cplusplus
}
#endif
