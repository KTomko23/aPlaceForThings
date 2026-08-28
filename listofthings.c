#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <time.h>

struct thing_node
{
    int data;
    struct thing_node *prev;
    struct thing_node *next;
};
int main()
{

    int *arr = malloc(sizeof(int) * 5);

    arr[0] = rand() % 100;
    arr[1] = rand() % 100;
    arr[2] = rand() % 100;
    arr[3] = rand() % 100;
    arr[4] = rand() % 100;

    printf("Array: ");
    for (int i = 0; i < 5; i++)
    {
        printf("%d ", arr[i]);
    }
    printf("\n");

    struct thing_node *head = malloc(sizeof(struct thing_node));
    struct thing_node *tail = malloc(sizeof(struct thing_node));
    struct thing_node *currThang = malloc(sizeof(struct thing_node));

    //always start off with 67
    head->data = 67;
    head->prev = tail;
    head->next = NULL;

    currThang = head;

    for (int i = 0; i < 5; i++)
    {

        struct thing_node *newThang = malloc(sizeof(struct thing_node));

        newThang->data = arr[i];

        newThang->prev = currThang;
        newThang->next = head;

        tail = newThang;
        currThang->next = newThang;
        currThang = newThang;
    }

    //print list

    printf("List: ");
    currThang = head;
    for (int i = 0; i < 6; i++)
    {
        printf("%d ", currThang->data);
        currThang = currThang->next;
    }
    printf("\n");

    return 0;
}