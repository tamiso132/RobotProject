#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/hci.h>
#include <bluetooth/hci_lib.h>
#include <string.h>

#include "scan.h"

#define MAX_DEVICES 10 // Maximum number of names

List *scan_devices()
{
    inquiry_info *ii = NULL;

    int max_rsp, num_rsp;
    int dev_id, sock, len, flags;
    int i;
    char addr[MAC_ADRESS_LENGTH] = {0};

    char name[MAX_NAME_LENGTH] = {0};

    List *devices = create_list();
    Device *device = malloc(sizeof(Device));

    printf("hello");

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

        strcpy(device->name, name);
        strcpy(device->mac_adress, addr);

        add_to_list(devices, (void *)device);
    }

    free(ii);
    close(sock);
    return devices;
}
