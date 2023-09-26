#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <errno.h>

#include <netinet/in.h>
#include <netdb.h>

#include <netinet/in.h>

#include "b_device.h"
#include "utility.h"

#define MAX_DEVICES 10 // Maximum number of names
#define BUF_SIZE 1024

#define PORT 40

int u_init_server()
{

    int server_fd, new_socket, valread;
    struct sockaddr_in address;
    int opt = 1;
    int addrlen = sizeof(address);
    char buffer[1024] = {0};
    char *hello = "Hello from server";

    // Creating socket file descriptor
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0)
    {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // Forcefully attaching socket to the port 8080
    if (setsockopt(server_fd, SOL_SOCKET,
                   SO_REUSEADDR | 15, &opt,
                   sizeof(opt)))
    {
        perror("setsockopt");
        exit(EXIT_FAILURE);
    }
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(PORT);

    // Forcefully attaching socket to the port 8080
    if (bind(server_fd, (struct sockaddr *)&address,
             sizeof(address)) < 0)
    {
        perror("bind failed");
        exit(EXIT_FAILURE);
    }
    if (listen(server_fd, 3) < 0)
    {
        perror("listen");
        exit(EXIT_FAILURE);
    }
    if ((new_socket = accept(server_fd, (struct sockaddr *)&address,
                             (socklen_t *)&addrlen)) < 0)
    {
        perror("accept");
        exit(EXIT_FAILURE);
    }
    valread = read(new_socket, buffer, 1024);
    printf("%s\n", buffer);
    send(new_socket, hello, strlen(hello), 0);
    printf("Hello message sent\n");

    // closing the connected socket
    close(new_socket);
    // closing the listening socket
    shutdown(server_fd, SHUT_RDWR);
    return 0;
}

#define SERVER_IP "127.0.0.1"

int u_device_connect(const char *ip_dress)
{
    int clilen, sockfd, newsockfd, n, cpid;
    char msg[100];
    struct sockaddr_in serv_addr, cli;
    if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0)
    {
        printf("socket failed to establish\n");
        exit(0);
    }
    printf("socket created\n");
    bzero((char *)&serv_addr, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    serv_addr.sin_port = htons(PORT);
    if (bind(sockfd, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0)
    {
        printf("binding failed\n");
        exit(0);
    }
    printf("binding established\n");
    if (listen(sockfd, 5) < 0)
    {
        printf("not listening\n");
        exit(0);
    }
    printf("listening|n");
    for (;;)
    {
        clilen = sizeof(cli);
        if ((newsockfd = accept(sockfd, (struct sockaddr *)&cli, &clilen)) < 0)
        {
            printf("accept failed\n");
            exit(0);
        }
        printf("accepted\n");
        cpid = fork();
        if (cpid == 0)
        {
            n = read(newsockfd, msg, 80);
            msg[n] = '\0';
            write(newsockfd, msg, strlen(msg));
            close(newsockfd);
            exit(0);
        }
    }
    return 0;
}
