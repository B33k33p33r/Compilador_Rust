pub fn get_runtime() -> String {
    r#"
#include <stdio.h>
#include <stdlib.h>

void print_int(long n) {
    printf("%ld\n", n);
}

void print_string(const char* s) {
    printf("%s\n", s);
}

long string_length(const char* s) {
    return strlen(s);
}

char* string_concat(const char* s1, const char* s2) {
    char* result = malloc(strlen(s1) + strlen(s2) + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}
"#
    .to_string()
}
