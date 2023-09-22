
#pragma once

typedef struct node
{
    void *data;
    struct node *next;
} Node;

struct list
{
    int size;
    Node *head;
} typedef List;

List *create_list();
void add_to_list(List *list, void *data);
void *remove_from_list(List *list);
void free_list(List *list);