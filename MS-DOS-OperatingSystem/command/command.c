/* command */
#include "command.h"

int main(int argc, char *argv[]) {
    init();

    /* Test store() */
    // int8 c;
    // c = store((int8)1);
    // printf("c = %d\n", $i c);
    // printf("errnumber = 0x%.02hhx\n", $i errnumber);

    /* Test load() */
    load((int8)1, 'X');
    printf("\n\n");

    /* Test disk emulator */
    dinit();

    return 0;
}