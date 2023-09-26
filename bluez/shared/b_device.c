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
#include <fcntl.h>

#include "b_device.h"
#include "utility.h"

#define MAX_DEVICES 10 // Maximum number of names

Device *test()
{
    printf("This C code is working.\n");
    Device *device = malloc(sizeof(Device));

    if (device == NULL)
    {
        // Handle memory allocation failure
        return NULL;
    }

    char *name = "heyoooo";
    char *mac = "0412490";

    // Allocate memory for the character arrays and copy the strings
    strcpy(device->name, name);
    strcpy(device->mac_address, mac);

    printf("This C code is working.\n");
    return device;
}

List *device_scan_bluetooth()
{
    inquiry_info *ii = NULL;
    puts("another msg");

    int max_rsp, num_rsp;
    int dev_id, sock, len, flags;
    int i;
    char addr[MAC_ADRESS_LENGTH] = {0};

    char name[MAX_NAME_LENGTH] = {0};

    List *devices = create_list();

    puts("hello");

    dev_id = hci_get_route(NULL);
    sock = hci_open_dev(dev_id);
    if (dev_id < 0 || sock < 0)
    {
        perror("opening socket");
        exit(1);
    }

    len = 8;
    max_rsp = 255;
    flags = IREQ_CACHE_FLUSH;
    ii = (inquiry_info *)malloc(max_rsp * sizeof(inquiry_info));

    num_rsp = hci_inquiry(dev_id, len, max_rsp, NULL, &ii, flags);
    if (num_rsp < 0)
        perror("hci_inquiry");

    for (i = 0; i < num_rsp; i++)
    {
        ba2str(&(ii + i)->bdaddr, addr);
        memset(name, 0, sizeof(name));
        if (hci_read_remote_name(sock, &(ii + i)->bdaddr, sizeof(name), name, 0) < 0)
            strcpy(name, "[unknown]");

        Device *device = malloc(sizeof(Device));

        strcpy(device->name, name);
        strcpy(device->mac_address, addr);

        add_to_list(devices, device);
    }

    puts("does it come here my friend\n");
    free(ii);
    close(sock);
    return devices;
}
int b_device_connect(const Device *device)
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
