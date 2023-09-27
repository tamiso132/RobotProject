#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/rfcomm.h>
#include <bluetooth/hci.h>
#include <bluetooth/hci_lib.h>
#include <errno.h>
#include "libusb.h"

#include "device.h"
#include "utility.h"

#define MAX_DEVICES 10 // Maximum number of names
#define BUF_SIZE 1024

List *u_device_scan()
{

    FILE *f;
    char *buf;

    f = popen("lsusb", "r");
    if (f == NULL)
    {
        perror("1 - Error");
        return errno;
    }

    buf = malloc(BUF_SIZE);
    if (buf == NULL)
    {
        perror("2 - Error");
        pclose(f);
        return errno;
    }

    while (fgets(buf, BUF_SIZE, f) != NULL)
    {
        printf("%s", buf);
    }
    puts("");

    pclose(f);
    free(buf);

    return 0;
}

int u_device_connect(const Device *device)
{
    struct sockaddr_rc addr = {0};
    addr.rc_family = AF_BLUETOOTH;
    addr.rc_channel = (uint8_t)1;

    str2ba(device->mac_address, &addr.rc_bdaddr);

    int s = socket(AF_BLUETOOTH, SOCK_STREAM, BTPROTO_RFCOMM);
    int status = connect(s, (struct sockaddr *)&device->mac_address, sizeof(device->mac_address));
    if (status == -1)
    {
        perror("unable to connect");
        return -1;
    }

    return s;
}
