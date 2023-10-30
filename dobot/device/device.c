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
    int f = write(socket, bytes, n);
    fsync(socket);
    return f;
}

int device_read(int socket, int bytes_to_read, uint8_t *buffer)
{
    int n = read(socket, buffer, bytes_to_read);
    return n;
}

int close_socket(int socket)
{
    close(socket);
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