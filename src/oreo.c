#include<stdio.h>
#include<stdlib.h>
struct node{
    int data;
    struct node *next;
};

int main(){
    char response = '`';
    struct node *head, *newnode, *temp;
    newnode = (struct node *)malloc(sizeof(struct node));
    if(newnode == NULL){
        printf("memory allocation failed...\n\n");
        newnode = (struct node *)malloc(sizeof(struct node));
    }
    head = newnode;


    while(1){
    printf("\nENTER DATA IN NODE = ");
    scanf("%d", &newnode->data);
    temp = (struct node *)malloc(sizeof(struct node));

    printf("ENTER RESPONSE (y/n) for (new node/end) respectively = ");
    scanf(" %c", &response);

    if(response == 'n' || response == 'N'){
        newnode->next = NULL;
        break;
    }
    else {
        newnode->next = temp;
        newnode = temp;
    }

    }


    int nodeno = 1;
    temp = head;
    while(temp != NULL){
        printf("\nDATA AT NODE NUMBER %d  is = %d", nodeno, temp->data);
        // printf("\n %d", temp->data);
        temp = temp->next;
        nodeno++;
    }
    nodeno--;
    printf("\n\ntotal no. of nodes present are = %d", nodeno);

printf("\n\n");

    int q = 0;
    while(1){
    printf("\nENTER 1 TO CREAT NEW NODE AT STARTING...");
    printf("\nENTER 2 TO CREAT NEW NODE AT IN BETWEEN NODES...");
    printf("\nENTER 3 TO CREAT NEW NODE AT END...");
    printf("\nENTER 4 TO exit...");
    printf("\nENTER RESPONSE = ");
    scanf(" %d", &q);

    switch(q){
    case 1:{
    response = '`';
        // do{
        newnode = (struct node *)malloc(sizeof(struct node));
        newnode->next = head;
        head = newnode;
        printf("\nENTER DATA IN NODE = ");
        scanf("%d", &newnode->data);
        // printf("ENTER Y FOR MORE NODES N TO EXIT = ");
        // scanf(" %c", &response);
        // }while(response != 'n' && response != 'N');
        break;
    }
    case 2:{
        int loc = 1, count = 1;
        printf("\nENTER LOCATION AFTER WHICH NEW NODE IS TO BE INSERTED = ");
        scanf(" %d", &loc);
        temp = head;
        while(count != loc){
            temp = temp->next;
            count++;
        }
        newnode = (struct node *)malloc(sizeof(struct node));
        newnode->next = temp->next;
        printf("\nENTER DATA IN NEW NODE = ");
        scanf("%d", &newnode->data);
        temp->next = newnode;
        break;
    }
    case 3:{
        temp = head;
        while(temp->next != NULL){
            temp = temp->next;
        }
        newnode = (struct node *)malloc(sizeof(struct node));
        printf("\nENTER DATA IN NEW NODE = ");
        scanf("%d", &newnode->data);
        temp -> next = newnode;
        newnode -> next = NULL;
        break;
    }
    case 4:{
        goto la;
    }

    default: 
    printf("INCORRECT RESPONSE...");
    break;


    }
    }

la:

printf("\n\n");
    nodeno = 1;
    temp = head;
    while(temp != NULL){
        printf("\nDATA AT NODE NUMBER %d  is = %d", nodeno, temp->data);
        // printf("\n %d", temp->data);
        temp = temp->next;
        nodeno++;
    }


printf("\n\n");
    return 0;
}