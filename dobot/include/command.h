#include <stdint.h>
typedef struct RobotPos
{
    float x;
    float y;
    float z;
    float r;
    float jointAngle[4]; // (basement, rear arm, forearm, endeffector) angles
} RobotPos;

RobotPos get_pose(int fd);
void reset_pose(int fd, float rear_arm_angle, float front_arm_angle);
