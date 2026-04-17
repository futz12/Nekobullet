#include <stdio.h>
#include <stdarg.h>

void* stdout = nullptr;
void* stderr = nullptr;

void printf(const char* fmt, ...) {
    (void)fmt;
}

int fflush(void* stream) {
    (void)stream;
    return 0;
}

int vsprintf_s(char* buffer, size_t sizeOfBuffer, const char* format, va_list argptr) {
    (void)sizeOfBuffer;
    (void)format;
    (void)argptr;
    if (buffer) {
        buffer[0] = '\0';
    }
    return 0;
}

int sprintf_s(char* buffer, size_t sizeOfBuffer, const char* format, ...) {
    (void)sizeOfBuffer;
    (void)format;
    if (buffer) {
        buffer[0] = '\0';
    }
    return 0;
}

int vsnprintf(char* str, size_t size, const char* format, va_list arg) {
    (void)size;
    (void)format;
    (void)arg;
    if (str) {
        str[0] = '\0';
    }
    return 0;
}

int snprintf(char* str, size_t size, const char* format, ...) {
    (void)size;
    (void)format;
    if (str) {
        str[0] = '\0';
    }
    return 0;
}
