#include <stdio.h>

int main() {
    char s[256];
    scanf("%254s", s);
    printf("%s\n", s);
    return 0;
}
