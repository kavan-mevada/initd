 #include <sys/prctl.h>
 #include <linux/capability.h>
 #include <sys/types.h>
 #include <unistd.h>
 #include <stdio.h>
 #include <stdlib.h>
 #include <string.h>

 #ifndef PR_CAPBSET_READ
 #define PR_CAPBSET_READ 23
 #endif

 #ifndef PR_CAPBSET_DROP
 #define PR_CAPBSET_DROP 24
 #endif

int usage(char *me)
{
	printf("Usage: %s get\n", me);
	printf("       %s drop <capability>\n", me);
	return 1;
}

 #define numcaps 32
char *captable[numcaps] = {
	"cap_chown",
	"cap_dac_override",
	"cap_dac_read_search",
	"cap_fowner",
	"cap_fsetid",
	"cap_kill",
	"cap_setgid",
	"cap_setuid",
	"cap_setpcap",
	"cap_linux_immutable",
	"cap_net_bind_service",
	"cap_net_broadcast",
	"cap_net_admin",
	"cap_net_raw",
	"cap_ipc_lock",
	"cap_ipc_owner",
	"cap_sys_module",
	"cap_sys_rawio",
	"cap_sys_chroot",
	"cap_sys_ptrace",
	"cap_sys_pacct",
	"cap_sys_admin",
	"cap_sys_boot",
	"cap_sys_nice",
	"cap_sys_resource",
	"cap_sys_time",
	"cap_sys_tty_config",
	"cap_mknod",
	"cap_lease",
	"cap_audit_write",
	"cap_audit_control",
	"cap_setfcap"
};

int getbcap(void)
{
	int comma=0;
	unsigned long i;
	int ret;

	printf("i know of %d capabilities\n", numcaps);
	printf("capability bounding set:");
	for (i=0; i<numcaps; i++) {
		ret = prctl(PR_CAPBSET_READ, i);
		if (ret < 0)
			perror("prctl");
		else if (ret==1)
			printf("%s%s", (comma++) ? ", " : " ", captable[i]);
	}
	printf("\n");
	return 0;
}

int capdrop(char *str)
{
	unsigned long i;

	int found=0;
	for (i=0; i<numcaps; i++) {
		if (strcmp(captable[i], str) == 0) {
			found=1;
			break;
		}
	}
	if (!found)
		return 1;
	if (prctl(PR_CAPBSET_DROP, i)) {
		perror("prctl");
		return 1;
	}
	return 0;
}

int main(int argc, char *argv[])
{
	if (argc<2)
		return usage(argv[0]);
	if (strcmp(argv[1], "get")==0)
		return getbcap();
	if (strcmp(argv[1], "drop")!=0 || argc<3)
		return usage(argv[0]);
	if (capdrop(argv[2])) {
		printf("unknown capability\n");
		return 1;
	}
	return execl("/bin/bash", "env", NULL);
}