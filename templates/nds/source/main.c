#include <nds.h>
#include <stdio.h>

int main(void)
{
    consoleDemoInit();

    iprintf("\n Hello World!");

    while (1)
    {
        swiWaitForVBlank();
    }

    return 0;
}
