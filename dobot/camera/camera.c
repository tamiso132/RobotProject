#include <signal.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include "device.h"

static pid_t pid = 0;

void takee_pic(char *filename)
{
    char cwd[500];

    if (getcwd(cwd, sizeof(cwd)) != NULL)
    {
        char buffer[256];
        sprintf(buffer, "%s/image", cwd);
        printf("%s\n", buffer);

        if ((pid == fork()) == 0)
        {

            execl("usr/bin/raspistill", "/usr/bin/raspistill", "-n", "-vf", "-o", buffer, NULL);
        }
    }
    else
    {
        perror("getcwd() error\n");
        return 1;
    }
}