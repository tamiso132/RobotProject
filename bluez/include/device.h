#pragma once
#include <stdint.h>
typedef struct List List;

#define MAX_NAME_LENGTH 248 // Maximum length of each name
#define MAC_ADRESS_LENGTH 18

typedef void (*Callback)(uint8_t *bytes, uint32_t size);

typedef struct Device
{
    char name[MAX_NAME_LENGTH];
    char mac_address[MAC_ADRESS_LENGTH];
} Device;

void device_write(int socket, uint8_t *bytes, int n);
int device_read(int socket, int bytes_to_read, uint8_t *bytes);

List *b_device_scan_bluetooth();

int b_device_connect(const Device *device);

List *u_device_scan();

int u_device_connect();

//red - power
// black - ground
// white rx -- usb port
//green tx -- out of the usb port

// serial port gives
//RX is 3.3 volt
//TX is 3.3 volt

//robot specs
