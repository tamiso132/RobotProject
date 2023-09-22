#ifndef SCAN_H_ /* Include guard */
#define SCAN_H_

#include "../utility/utility.h"

#define MAX_NAME_LENGTH 248 // Maximum length of each name
#define MAC_ADRESS_LENGTH 18

struct Device
{
    char name[MAX_NAME_LENGTH];
    char mac_adress[MAC_ADRESS_LENGTH];
} typedef Device;

List *scan_devices();

#endif