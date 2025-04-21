#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>

int scan_port(const char *ip, int port) {
		int sock;
		struct sockaddr_in target;
		
		sock = socket(AF_INET, SOCK_STREAM, 0);
		if (sock < 0) {
			perror("Socket creation failed !!!");
			return 0;
		}
		
		target.sin_family = AF_INET;
		target.sin_port = htons(port);
		target.sin_addr.s_addr = inet_addr(ip);
		
		int result = connect(sock, (struct sockaddr *)&target, sizeof(target));
		close(sock);
		
		return ( result == 0);
	}
	
int main(int argc, char *argv[]) {
	if (argc != 4) {
		printf("Usage: %s <IP> <start port> <end port>\n", argv[0]);
		return 1;
	}
	
	const char *ip = argv[1];
	int start_port = atoi(argv[2]);
	int end_port = atoi(argv[3]);

	if (start_port < 1|| end_port > 65535 || start_port > end_port) {
		fprintf(stderr, "Invalid port range.\n");
		return 1;	
	}

	printf("Scanning %s from port %d to %d...\n\n", ip, start_port, end_port);
	
	for (int port = start_port; port <= end_port; port++) {
		if (scan_port(ip, port)) {
			printf("[+] Port %d is OPEN\n", port);
		}
	}

	printf("\nScan Completed.\n");
	
	return 0;	
}
