pub fn get_runtime() -> String {
    r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__declspec(dllexport) void print_int(long long n) {
    printf("%lld\n", n);
}

__declspec(dllexport) void print_string(const char* s) {
    printf("%s\n", s);
}

__declspec(dllexport) long long string_length(const char* s) {
    return strlen(s);
}

__declspec(dllexport) char* string_concat(const char* s1, const char* s2) {
    char* result = malloc(strlen(s1) + strlen(s2) + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}
"#
    .to_string()
}
