#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <errno.h>

#include "b_device.h"
#include "utility.h"

#define MAX_DEVICES 10 // Maximum number of names
#define BUF_SIZE 1024

#define PORT 12345

int u_init_server()
{

    int server_socket, client_socket;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_addr_len = sizeof(client_addr);

    // Create socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket == -1)
    {
        perror("Socket creation error");
        exit(1);
    }

    // Configure server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Bind socket to the server address
    if (bind(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1)
    {
        perror("Bind error");
        close(server_socket);
        exit(1);
    }

    // Listen for incoming connections
    if (listen(server_socket, 5) == -1)
    {
        perror("Listen error");
        close(server_socket);
        exit(1);
    }

    printf("Server is listening on port %d...\n", PORT);

    // Accept incoming connection
    client_socket = accept(server_socket, (struct sockaddr *)&client_addr, &client_addr_len);
    if (client_socket == -1)
    {
        perror("Accept error");
        close(server_socket);
        exit(1);
    }
    return server_socket;
}

int u_device_connect(char *ip_dress)
{
    int client_socket;
    struct sockaddr_in server_addr;

    // Create socket
    client_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (client_socket == -1)
    {
        perror("Socket creation error");
        exit(1);
    }

    // Configure server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = inet_addr(ip_dress); // Replace with the server's IP address
    server_addr.sin_port = htons(PORT);

    // Connect to the server
    if (connect(client_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1)
    {
        perror("Connection error");
        close(client_socket);
        exit(1);
    }

    printf("Connected to server %s:%d\n", ip_dress, PORT);

    return client_socket;
}
