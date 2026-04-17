#pragma once

#include <stdlib.h>
#include <stdint.h>

void *memset(void *dest, int c, size_t n);

void *memcpy(void *__restrict dest, const void *__restrict src, size_t n);

int strncmp(const char *s1, const char *s2, size_t n);

void *memmove(void *dest, const void *src, size_t n);

size_t strlen(const char *s);

char* strcpy(char* dest, const char* src);

char* strncpy(char* dest, const char* src, size_t n);

int strcmp(const char *s1, const char *s2);

char* strcat(char* dest, const char* src);

char* strchr(const char* s, int c);
