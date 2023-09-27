#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include "device.h"
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
    while (1)
    {
        // Read data from the serial port
        ssize_t bytesRead = read(fd, buffer, sizeof(buffer));
        if (bytesRead > 0)
        {
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