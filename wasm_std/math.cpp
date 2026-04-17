extern "C" {

float bw_sqrtf(float x) {
    return __builtin_sqrtf(x);
}

float bw_fabsf(float x) {
    return __builtin_fabsf(x);
}

float bw_cosf(float x) {
    return __builtin_cosf(x);
}

float bw_sinf(float x) {
    return __builtin_sinf(x);
}

float bw_tanf(float x) {
    return __builtin_tanf(x);
}

float bw_acosf(float x) {
    return __builtin_acosf(x);
}

float bw_asinf(float x) {
    return __builtin_asinf(x);
}

float bw_atanf(float x) {
    return __builtin_atanf(x);
}

float bw_atan2f(float y, float x) {
    return __builtin_atan2f(y, x);
}

float bw_expf(float x) {
    return __builtin_expf(x);
}

float bw_logf(float x) {
    return __builtin_logf(x);
}

float bw_powf(float x, float y) {
    return __builtin_powf(x, y);
}

float bw_fmodf(float x, float y) {
    return __builtin_fmodf(x, y);
}

double bw_floor(double x) {
    return __builtin_floor(x);
}

double bw_ceil(double x) {
    return __builtin_ceil(x);
}

double bw_sqrt(double x) {
    return __builtin_sqrt(x);
}

double bw_fabs(double x) {
    return __builtin_fabs(x);
}

double bw_cos(double x) {
    return __builtin_cos(x);
}

double bw_sin(double x) {
    return __builtin_sin(x);
}

double bw_tan(double x) {
    return __builtin_tan(x);
}

double bw_acos(double x) {
    return __builtin_acos(x);
}

double bw_asin(double x) {
    return __builtin_asin(x);
}

double bw_atan(double x) {
    return __builtin_atan(x);
}

double bw_atan2(double y, double x) {
    return __builtin_atan2(y, x);
}

double bw_exp(double x) {
    return __builtin_exp(x);
}

double bw_log(double x) {
    return __builtin_log(x);
}

double bw_pow(double x, double y) {
    return __builtin_pow(x, y);
}

double bw_fmod(double x, double y) {
    return __builtin_fmod(x, y);
}

bool bw_isnan(double x) {
    return __builtin_isnan(x);
}

bool bw_isinf(double x) {
    return __builtin_isinf(x);
}

}
