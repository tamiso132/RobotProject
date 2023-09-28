#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include "device.h"

#define SERIAL_PORT_PATH "/dev/ttyS0\0"

int configure(int fd);

uint8_t calculateChecksum(uint8_t *data, int len)
{
    uint8_t checksum = 0;
    for (int i = 0; i < len; i++)
    {
        checksum += data[i];
    }
    return ~checksum; // Take two's complement
}

int file_open_and_get_descriptor(const char *fname)
{
    int fd;
    fd = open(fname, O_RDWR | O_NONBLOCK);

    if (fd < 0)
    {
        printf("Could not open file");
    }
    configure(fd);
    return fd;
}

int open_serial_port(void)
{
    int fd = file_open_and_get_descriptor(SERIAL_PORT_PATH);
    return fd;
}

int configure(int fd)
{
    // Configure the serial port
    struct termios serialConfig;
    memset(&serialConfig, 0, sizeof(serialConfig));
    if (tcgetattr(fd, &serialConfig) != 0)
    {
        perror("Error getting serial port attributes");
        close(fd);
        return -1;
    }

    cfsetispeed(&serialConfig, B115200); // Baud rate
    cfsetospeed(&serialConfig, B115200);
    serialConfig.c_cflag &= ~PARENB; // No parity
    serialConfig.c_cflag &= ~CSTOPB; // 1 stop bit
    serialConfig.c_cflag &= ~CSIZE;
    serialConfig.c_cflag |= CS8; // 8 data bits

    if (tcsetattr(fd, TCSANOW, &serialConfig) != 0)
    {
        perror("Error setting serial port attributes");
        close(fd);
        return -1;
    }
    return 0;
}

int test()
{
    const char *port = "/dev/ttyUSB0"; // Replace with your serial port name

    // Open the serial port
    int fd = open(port, O_RDWR | O_NOCTTY);
    if (fd == -1)
    {
        perror("Error opening serial port");
        return 1;
    }

    // Configure the serial port
    struct termios serialConfig;
    memset(&serialConfig, 0, sizeof(serialConfig));
    if (tcgetattr(fd, &serialConfig) != 0)
    {
        perror("Error getting serial port attributes");
        close(fd);
        return 1;
    }

    // Set baud rate, data bits, stop bits, and parity
    cfsetispeed(&serialConfig, B115200); // Baud rate
    cfsetospeed(&serialConfig, B115200);
    serialConfig.c_cflag &= ~PARENB; // No parity
    serialConfig.c_cflag &= ~CSTOPB; // 1 stop bit
    serialConfig.c_cflag &= ~CSIZE;
    serialConfig.c_cflag |= CS8; // 8 data bits

    // Apply the new settings
    if (tcsetattr(fd, TCSANOW, &serialConfig) != 0)
    {
        perror("Error setting serial port attributes");
        close(fd);
        return 1;
    }

    // Read and write data
    char buffer[256];

    // Construct the packet
    uint8_t header[] = {0xAA, 0xAA};
    uint16_t len = 2 + strlen("DeviceSN"); // Header length + payload length
    uint8_t id = 0;
    uint8_t ctrl = 0x10; // For example, read operation (0x10) and not queued
    char deviceSN[] = "DeviceSN";
    uint8_t protocol = 3;

    // Calculate the checksum
    uint8_t payload[len - 4]; // Exclude header, len, and checksum bytes from the payload
    memcpy(payload, &id, 1);
    memcpy(payload + 1, &ctrl, 1);
    memcpy(payload + 2, deviceSN, strlen(deviceSN));
    memcpy(payload + strlen(deviceSN) + 2, &protocol, 1);
    uint8_t checksum = calculateChecksum(payload, len - 4); // Exclude header, len, and checksum bytes

    // Construct the complete packet
    uint8_t packet[len];
    memcpy(packet, header, 2);
    memcpy(packet + 2, &len, 2);
    memcpy(packet + 4, &id, 1);
    memcpy(packet + 5, &ctrl, 1);
    memcpy(packet + 6, deviceSN, strlen(deviceSN));
    memcpy(packet + 6 + strlen(deviceSN), &protocol, 1);
    memcpy(packet + len - 1, &checksum, 1);

    // int64_t x = 0xAAAA020A00F6;
    int64_t x = 0xF600A020AAAA;

    // header - first byte
    // Len - second bits
    // ID - third byte
    // CTRL - fourth byte // 4 bits RW, 4 bits isQueue
    // checksum - fifth byte

    printf("hello\n");
    // Send the packet
    write(fd, &x, 8);

    while (1)
    {
        // Read data from the serial port
        ssize_t bytesRead = read(fd, buffer, sizeof(buffer));
        printf("hello\n");

        if (bytesRead > 0)
        {
            printf("hello\n");
            printf("Received: %.*s", (int)bytesRead, buffer);
        }

        // Write data to the serial port
        const char *message = "Hello, Serial!\n";
        ssize_t bytesWritten = write(fd, message, strlen(message));
        if (bytesWritten == -1)
        {
            perror("Error writing to serial port");
        }

        usleep(1000000); // Sleep for 1 second
    }

    // Close the serial port (not reached in this example)
    close(fd);
    return 0;
}

// black --> ground
// red --> power 5v
// white --> RX
// green --> TX