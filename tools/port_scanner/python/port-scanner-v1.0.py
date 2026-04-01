#!/usr/bin/env python3

import socket

def scan_port(ip, port):
	try:
		sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		sock.settimeout(1)
		result = sock.connect_ex((ip, port))
		if result == 0:
			print(f"[+] Port {port} is OPEN")
		sock.close()
	except KeyboardInterrupt:
		print("\n[!] Exiting")
		exit()
	except socket.error:
		print("[!] Couldn't connect to server")
		exit()
	
def main():
	target_ip = input("Enter target IP: ")
	start_port = int(input("Start port: "))
	end_port = int(input("End port: "))
	
	print(f"\nScanning {target_ip} from port {start_port} to {end_port}...\n")
	for port in range(start_port, end_port + 1):
		scan_port(target_ip, port)
		
if __name__ == '__main__':
	main()
