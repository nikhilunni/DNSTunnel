#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <sys/sys_domain.h>
#include <sys/kern_control.h>
#include <net/if_utun.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

int open_utun(uint8_t* tun_id) {
    // Open socket for use
    int socket_fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);

    struct ctl_info info;

    if (strlcpy(info.ctl_name, UTUN_CONTROL_NAME, sizeof(info.ctl_name)) >= sizeof(info.ctl_name)) {
        fprintf(stderr,"UTUN_CONTROL_NAME too long");
        return -1;
    }

    if (ioctl(socket_fd, CTLIOCGINFO, &info) == -1) {
        perror("ioctl(CTLIOCGINFO)");
        close(socket_fd);
        return -1;
    }

    struct sockaddr_ctl addr;
    addr.sc_id = info.ctl_id;
    addr.sc_len = sizeof(addr);
    addr.sc_family = AF_SYSTEM;
    addr.ss_sysaddr = AF_SYS_CONTROL;

    int err = 0;

    do {
        addr.sc_unit = *tun_id + 1;
        err = connect(socket_fd, (struct sockaddr *)&addr, sizeof(addr));

        if (err < 0) {
            *tun_id += 1;
        }
    } while (err < 0 && *tun_id < 255);

    if (err != 0) {
        close(socket_fd);
        return -1;
    } else {
        return socket_fd;
    }
}
