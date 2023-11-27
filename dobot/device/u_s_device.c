// C library headers
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>

// Linux headers
#include <sys/file.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/ioctl.h>
#include <fcntl.h>   // Contains file controls like O_RDWR
#include <errno.h>   // Error integer and strerror() function
#include <termios.h> // Contains POSIX terminal control definitions
#include <unistd.h>  // write(), read(), close()

#include "device.h"

#define SERIAL_PORT_PATH "/dev/ttyS0\0"

int configure(int fd);
int configure_serial(int fd);

int *file_open_and_get_descriptor(const char *fname)
{
    // Open the serial port
    int fd = open("/dev/ttyUSB0", O_RDWR | O_NOCTTY | O_NDELAY);

    // FILE *file = fopen("/dev/ttyUSB1", O_RDWR | O_NOCTTY | O_NDELAY);
    if (fd == -1)
    {
        perror("Error opening serial port");
        return 1;
    }
    fcntl(fd, F_SETFL, 0);
    return fd;
}

int open_serial_port(void)
{
    int fd = file_open_and_get_descriptor("f");
    configure_serial(fd);
    return fd;
}

int set_signal(int param, int fd)
{
    int stat_;

    if (fd == -1)
        return;

    if (ioctl(fd, TIOCMGET, &stat_) == -1)
    {
        return;
    }

    /* DTR */
    if (param == 0)
    {
        if (stat_ & TIOCM_DTR)
            stat_ &= ~TIOCM_DTR;
        else
            stat_ |= TIOCM_DTR;
    }
    /* RTS */
    else if (param == 1)
    {
        if (stat_ & TIOCM_RTS)
            stat_ &= ~TIOCM_RTS;
        else
            stat_ |= TIOCM_RTS;
    }
}

int configure_serial(int fd)
{
    struct termios options;
    memset(&options, 0, sizeof(options));
    if (tcgetattr(fd, &options) != 0)
    {
        perror("Error getting serial port attributes\n");
        close(fd);
        return 1;
    }

    if (flock(fd, LOCK_EX | LOCK_NB) == -1)
    {
        printf("Cannot lock port\n");
        return -1;
    }

    // Set baud rate, data bits, stop bits, and parity
    cfsetispeed(&options, B115200); // Baud rate
    cfsetospeed(&options, B115200);

    options.c_cflag = B115200;

    options.c_cflag &= ~CSIZE;
    options.c_cflag |= CS8;
    options.c_iflag &= ~(INPCK | PARMRK | ISTRIP);
    options.c_cflag &= ~CSTOPB;

    options.c_cflag |= CREAD | CLOCAL;
    options.c_cc[VMIN] = 1;
    options.c_cc[VTIME] = 0;

    // options.c_cflag &= ~CRTSCTS;        // turn off hardware flow control
    options.c_iflag &= ~(IXON | IXOFF); // turn off sowftware flow control

    // options.c_lflag &= ~ICANON;
    // options.c_lflag &= ~ISIG;
    // options.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL);
    // options.c_oflag &= ~OPOST;
    // options.c_oflag &= ~ONLCR;

    printf("hello\n");
    options.c_oflag = 0;
    options.c_lflag = 0;

    tcflush(fd, TCIFLUSH);
    tcflush(fd, TCOFLUSH);
    tcsetattr(fd, TCSANOW, &options);

    printf("Configuration done of serial\n");
}
