// C library headers
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Linux headers
#include <fcntl.h>   // Contains file controls like O_RDWR
#include <errno.h>   // Error integer and strerror() function
#include <termios.h> // Contains POSIX terminal control definitions
#include <unistd.h>  // write(), read(), close()
#include "device.h"
#include "command.h"

#define SERIAL_PORT_PATH "/dev/ttyS0\0"

int configure(int fd);

int file_open_and_get_descriptor(const char *fname)
{
    const char *port = "/dev/ttyUSB0"; // Replace with your serial port name

    // Open the serial port
    int fd = open(port, O_RDWR | O_NOCTTY | O_NDELAY);
    fcntl(fd, F_SETFL, 0);
    if (fd == -1)
    {
        perror("Error opening serial port");
        return 1;
    }
    return fd;
}

int open_serial_port(void)
{
    int fd = file_open_and_get_descriptor(SERIAL_PORT_PATH);
    configure_serial(fd);
    return fd;
}

int configure_serial(int fd)
{
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
    serialConfig.c_cflag &= ~CSIZE;  // Clear the data bits field
    serialConfig.c_cflag |= CS8;     // Set 8 data bits
    serialConfig.c_cflag &= ~PARENB; // No parity
    serialConfig.c_iflag &= ~(INPCK | PARMRK | ISTRIP);
    serialConfig.c_cflag &= ~CSTOPB; // 1 stop bit

    serialConfig.c_cflag |= CREAD | CLOCAL;
    serialConfig.c_cc[VMIN] = 0;
    serialConfig.c_cc[VTIME] = 10;

    serialConfig.c_cflag &= ~CRTSCTS;                // turn off flow control hw
    serialConfig.c_cflag &= ~(IXON | IXOFF | IXANY); // turn off flow control sw
    serialConfig.c_lflag &= ~ICANON;                 // Disable canonical mode
    serialConfig.c_lflag &= ~ISIG;                   // Disable signal generation
    serialConfig.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL);
    serialConfig.c_iflag &= ~OPOST; // prevent special interpretation
    serialConfig.c_iflag &= ~ONLCR; // prevent conversion of newline

    // Apply the new settings
    tcflush(fd, TCIFLUSH);
    if (tcsetattr(fd, TCSANOW, &serialConfig) != 0)
    {
        perror("Error setting serial port attributes");
        close(fd);
        return 1;
    }
}
int test_read()
{
    const char *port = "/dev/ttyUSB0"; // Replace with your serial port name

    // Open the serial port
    int fd = open(port, O_RDWR | O_NOCTTY);
    if (fd == -1)
    {
        perror("Error opening serial port");
        return 1;
    }
}

int test()
{
    int fd = open_serial_port();
   // reset_pose(fd, 60.0, 60.0);
    RobotPos pos = get_pose(fd);
}

// black --> ground
// red --> power 5v
// white --> RX
// green --> TX