#include <stdio.h>
#include <stdlib.h>
#include "utility.h"
#include "device.h"

List *create_list()
{
    List *new_list = (List *)malloc(sizeof(List));
    new_list->size = 0;
    new_list->head = NULL;
    return new_list;
}

void add_to_list(List *list, Device *data)
{
    Node *new_node = (Node *)malloc(sizeof(Node));
    new_node->data = data;
    new_node->next = list->head;
    list->head = new_node;
    list->size++;
}

Device *pop_front(List *list)
{
    if (list->size == 0)
    {
        return NULL;
    }
    Node *node_to_remove = list->head;
    Device *data = node_to_remove->data;
    list->head = node_to_remove->next;
    free(node_to_remove);
    list->size--;
    return data;
}

void free_list(List *list)
{
    Node *current_node = list->head;
    while (current_node != NULL)
    {
        Node *next_node = current_node->next;
        free(current_node);
        current_node = next_node;
    }
    free(list);
}
