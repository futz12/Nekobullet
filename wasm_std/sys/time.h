#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int64_t time_t;
typedef int64_t suseconds_t;

struct timeval {
    time_t tv_sec;
    suseconds_t tv_usec;
};

struct timezone {
    int tz_minuteswest;
    int tz_dsttime;
};

inline int gettimeofday(struct timeval* tv, struct timezone* tz) {
    (void)tz;
    if (tv) {
        tv->tv_sec = 0;
        tv->tv_usec = 0;
    }
    return 0;
}

#ifdef __cplusplus
}
#endif
