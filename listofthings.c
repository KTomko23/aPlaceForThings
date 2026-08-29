#include <stdio.h>
#include <stdlib.h>
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

    struct thing_node *headThang = malloc(sizeof(struct thing_node));
    struct thing_node *tailThang = malloc(sizeof(struct thing_node));

    //always start off with 67
    headThang->data = 67;

    tailThang = headThang;

    for (int i = 0; i < 5; i++)
    {

        struct thing_node *newThang = malloc(sizeof(struct thing_node));

        newThang->data = arr[i];

        newThang->prev = tailThang;
        newThang->next = headThang;

        tailThang->next = newThang;

        tailThang = newThang;
    }

    headThang->prev = tailThang;

    //print list

    printf("List: ");
    for (int i = 0; i < 6; i++)
    {
        printf("%d ", headThang->data);
        headThang = headThang->next;
    }
    printf("\n");

    //circular 

    printf("Circular: ");
    for (int i = 0; i < 18; i++)
    {
        printf("%d ", headThang->data);
        headThang = headThang->next;
    }
    printf("\n");

    printf("Reverse: ");
    for (int i = 0; i < 18; i++)
    {
        printf("%d ", tailThang->data);
        tailThang = tailThang->prev;
    }
    printf("\n");

    return 0;
}