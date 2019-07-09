#include <stdio.h>
#include <stdlib.h>

#define CHARS 2
static char PARENS[CHARS] = "()";
static int COST[CHARS] = {1, -1};

void gen(int i, int n, int count, char *buf) {
    if (i == n) {
        if (count == 0) {
            printf("%s\n", buf);
        }
        return;
    }

    for (int j = 0; j < CHARS; ++j) {
        int new_count = count + COST[j];
        if (new_count >= 0) {
            buf[i] = PARENS[j];
            gen(i+1, n, new_count, buf);
        }
    }
}

int main(void) {
    int n;
    char *buf;

    scanf("%d", &n);
    buf = calloc(2*n+1, sizeof(char));
    gen(0, 2*n, 0, buf);
    return 0;
}
