#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/rfcomm.h>
#include "../shared/scan.h"

int main(int argc, char **argv)
{

    struct sockaddr_rc addr = {0};
    int s, status;
    List *device_list = scan_devices();
    printf("\ndevices size: %d", device_list->size);
    Device *device = (Device *)remove_from_list(device_list);
    while (strcmp(device->name, "tommers") != 0 && device_list->size > 0)
    {
        printf("\n%s\n", device->mac_adress);
        printf("\n%s\n", device->name);
        printf("REMOVED");
        free(device);
        device = (Device *)remove_from_list(device_list);
    }

    if (device == NULL)
    {
        printf("\nCANCELED\n");
        return -1;
    }

    free_list(device_list);

    // allocate a socket
    s = socket(AF_BLUETOOTH, SOCK_STREAM, BTPROTO_RFCOMM);

    // set the connection parameters (who to connect to)
    addr.rc_family = AF_BLUETOOTH;
    addr.rc_channel = (uint8_t)1;

    printf("\n%s\n", device->mac_adress);
    printf("\n%s\n", device->name);
    str2ba(device->mac_adress, &addr.rc_bdaddr);

    // connect to server
    status = connect(s, (struct sockaddr *)&addr, sizeof(addr));

    // send a message
    if (status == 0)
    {
        status = write(s, "hello!", 6);
    }

    if (status < 0)
        perror("uh oh");

    close(s);
    return 0;
}