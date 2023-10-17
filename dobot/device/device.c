#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <errno.h>
#include <fcntl.h>

#include "device.h"
#include "utility.h"

int device_write(int socket, uint8_t *bytes, int n)
{
    printf("n bytes to write: %d\n", n);
    for (int i = 0; i < n; i++)
    {
        printf("%d\n", bytes[i]);
    }
    return write(socket, bytes, n);
}

int device_read(int socket, int bytes_to_read, uint8_t *buffer)
{
    printf("n bytes to read: %d\n", bytes_to_read);
    int n = read(socket, buffer, bytes_to_read);
    printf("Error code: %s\n", strerror(errno));
    return n;
}

int device_non_blocking_read(int socket, uint8_t *buffer, int n)
{
    int flags = fcntl(socket, F_GETFL, 0);
    flags |= O_NONBLOCK;
    int status = fcntl(socket, F_SETFL, flags);
    if (status == -1)
    {
        return errno;
    }

    int bytes_read = read(socket, buffer, n);

    if (bytes_read == -1)
    {
        return errno;
    }

    return bytes_read;
}