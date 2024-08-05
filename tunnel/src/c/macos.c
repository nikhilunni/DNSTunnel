#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <sys/sys_domain.h>
#include <sys/kern_control.h>
#include <net/if_utun.h>
#include <net/if.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

int init_interface(const uint8_t tun_id) {
    int socket_fd = socket(AF_INET, SOCK_DGRAM, 0);

    if (socket_fd < 0) {
        perror("socket(AF_INET, SOCK_DGRAM, 0)");
        return -1;
    }

    // Get utun name
    char *tun_name_fmt = "utun%d";
    int tun_name_len = snprintf(NULL, 0, tun_name_fmt, tun_id);

    struct ifreq ifr;
    snprintf(ifr.ifr_name, IFNAMSIZ, tun_name_fmt, tun_id);

    fprintf(stdout, "Interface name: %s\n", ifr.ifr_name);

    struct sockaddr_in *addr = (struct sockaddr_in *)&ifr.ifr_addr;
    addr->sin_family = AF_INET;

    // Set IP address
    addr->sin_addr.s_addr = inet_addr("10.0.0.1");
    if (ioctl(socket_fd, SIOCSIFADDR, &ifr) < 0) {
        perror("ioctl SIOCSIFADDR");
        close(socket_fd);
        return -1;
    }

    // Set point-to-point address
    addr->sin_addr.s_addr = inet_addr("10.0.0.2");
    if (ioctl(socket_fd, SIOCSIFDSTADDR, &ifr) < 0) {
        perror("ioctl SIOCSIFDSTADDR");
        close(socket_fd);
        return -1;
    }

    // Set netmask
    addr->sin_addr.s_addr = inet_addr("255.255.255.0");
    if (ioctl(socket_fd, SIOCSIFNETMASK, &ifr) < 0) {
        perror("ioctl SIOCSIFNETMASK");
        close(socket_fd);
        return -1;
    }

    // Bring the interface up
    ifr.ifr_flags |= IFF_UP;
    if (ioctl(socket_fd, SIOCSIFFLAGS, &ifr) < 0) {
        perror("ioctl SIOCSIFFLAGS");
        close(socket_fd);
        return -1;
    }

    close(socket_fd);
    return 0;
}

int open_utun(uint8_t* tun_id) {
    // Open socket for use
    int socket_fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);

    if (socket_fd < 0) {
        perror("socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL)");
        return -1;
    }

    // Open utun
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

    // Keep trying to connect to utun{tun_id} until we find an available one
    do {
        addr.sc_unit = *tun_id + 1;
        err = connect(socket_fd, (struct sockaddr *)&addr, sizeof(addr));

        if (err < 0) {
            *tun_id += 1;
        }
    } while (err < 0 && *tun_id < 255);

    if (err < 0) {
        close(socket_fd);
        return -1;
    }

    err = init_interface(*tun_id);

    if (err < 0) {
        fprintf(stderr, "Failed to initialize interface\n");
        close(socket_fd);
        return -1;
    }

    return socket_fd;
}
