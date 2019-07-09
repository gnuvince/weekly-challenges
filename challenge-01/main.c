#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

struct Node {
    int32_t n;
    struct Node *next;
};

void print_list(struct Node *curr) {
    while (curr != NULL) {
        printf("%" PRIi32 " ", curr->n);
        curr = curr->next;
    }
    printf("\n");
}

struct Node * create_list(int n) {
    struct Node *next = NULL;
    while (n != 0) {
        struct Node *curr = malloc(sizeof *curr);
        curr->n = n;
        curr->next = next;
        next = curr;
        n -= 1;
    }
    return next;
}

size_t length_list(struct Node *curr) {
    size_t len = 0;
    while (curr != NULL) {
        len += 1;
        curr = curr->next;
    }
    return len;
}

struct Node * midpoint_list(struct Node *curr) {
    size_t mid = (length_list(curr) + 1) / 2;
    while (mid != 0) {
        curr = curr->next;
        mid -= 1;
    }
    return curr;
}

void cut_list(struct Node *curr, struct Node *cutoff) {
    if (curr == NULL)
        return;

    while (curr != NULL && curr->next != cutoff) {
        curr = curr->next;
    }
    curr->next = NULL;
}

struct Node * reverse_list(struct Node *curr) {
    struct Node *prev = NULL;
    while (curr != NULL) {
        struct Node *next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}

void merge_lists(struct Node *first, struct Node *second) {
    while (first != NULL && second != NULL) {
        struct Node *first_next = first->next;
        struct Node *second_next = second->next;

        first->next = second;
        second->next = first_next;

        first = first_next;
        second = second_next;
    }
}

void dealloc_list(struct Node *curr) {
    while (curr != NULL) {
        struct Node *next = curr->next;
        free(curr);
        curr = next;
    }
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <list length>\n", argv[0]);
        return 1;
    }

    size_t len = atoi(argv[1]);
    struct Node *first = create_list(len);
    struct Node *second = midpoint_list(first);
    cut_list(first, second);
    struct Node *second_rev = reverse_list(second);
    merge_lists(first, second_rev);
    print_list(first);
    dealloc_list(first);
    return 0;
}
