#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// Linux headers
#include <fcntl.h>   // Contains file controls like O_RDWR
#include <errno.h>   // Error integer and strerror() function
#include <termios.h> // Contains POSIX terminal control definitions
#include <unistd.h>  // write(), read(), close()

#include "command.h"

struct __attribute__((packed)) __response_header
{
    uint16_t header;
    uint8_t len;
    uint8_t id;
    uint8_t ctrl;
};

#define RESPONSE_STRUCT(name, body)                   \
    struct __attribute__((packed)) name##Response     \
    {                                                 \
        struct __response_header info;                \
        struct name##; /* also checksum thing here */ \
        uint8_t checksum;                             \
    };

// RESPONSE_STRUCT(GetPose, { float x; float y; float z; float r; float join_angle[4]; });

// struct __attribute__((packed)) GetPoseResponse
// {
//     struct __response_header info;
//     struct GetPose
//     {
//         float x;
//         float y;
//         float z;
//         float r;
//         float join_angle[4];
//     };
//     uint8_t checksum;
// };

uint8_t calculateChecksum(uint8_t *data, int len)
{
    uint8_t checksum = 0;
    for (int i = 0; i < len; i++)
    {
        checksum += data[i];
    }
    return ~checksum + 1; // Take two's complement
}

void reset_pose(int fd, float rear_arm_angle, float front_arm_angle)
{
    uint8_t payload[] = {11, 0x10, 1, rear_arm_angle, front_arm_angle};
    uint8_t checksum = calculateChecksum(payload, sizeof(payload));
    uint8_t command[] = {
        170,
        170,
        11,
        0x10,
        1,
        rear_arm_angle,
        front_arm_angle,
        checksum,
    };
    ssize_t bytes_written = write(fd, command, sizeof(command));
    if (bytes_written < 0)
    {
        perror("Error writing to serial port\n");
        return;
    }
    uint8_t buffer[36];

    ssize_t bytes_read = read(fd, buffer, sizeof(buffer));

    if (bytes_read == 0)
    {
        printf("Error code: %s\n", strerror(0));
    }
}
struct RobotPos get_pose(int fd)
{
    uint8_t command[] = {
        170,
        170,
        6,
        31,
        3,
        0,
        0,
        0,
        0,
        222};

    ssize_t bytes_written = write(fd, command, sizeof(command));
    if (bytes_written < 0)
    {
        perror("Error writing to serial port\n");
        return;
    }

    uint8_t buffer[50];

    ssize_t bytes_read = read(fd, buffer, sizeof(buffer));

    if (bytes_read == -1)
    {
        printf("Error code: %s\n", strerror(errno));
    }
    else
    {
        // struct GetPoseResponse respons;
        // memcpy(&respons, buffer, sizeof(struct GetPoseResponse));
        // uint8_t header = respons.info.header;
        // uint8_t len = respons.info.len;
        // uint8_t ctrl = respons.info.ctrl;
        // printf("Header:%x\nLen:%f\nctrl", header, len, ctrl);
        return;
    }
}