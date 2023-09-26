#pragma once

typedef struct Device Device;

typedef struct node
{
    Device *data;
    struct node *next;
} Node;

typedef struct List
{
    int size;
    Node *head;
} List;

List *create_list();
void add_to_list(List *list, Device *data);
Device *pop_front(List *list);
void free_list(List *list);