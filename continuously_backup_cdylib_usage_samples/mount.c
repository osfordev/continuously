//
// gcc -g mount.c -o mount -lstratum -L../target/debug && LD_LIBRARY_PATH=../target/debug ./mount
//

#include "../continuously_backup_cdylib/bindings_c.h"

int main() {
    continuously_backup_mount();
}