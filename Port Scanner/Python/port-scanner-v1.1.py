#!/usr/bin/env python3

import socket
import threading
from concurrent.futures import ThreadPoolExecutor

print_lock = threading.Lock()

def scan(ip, port):
	try:
		sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		sock.settimeout(0.5)
		result = sock.connect_ex((ip, port))
		if result == 0:
			with print_lock:
				print(f"[+] Port {port} is OPEN")
		sock.close()
	except KeyboardInterrupt:
		print("\n[!] Exiting")
		exit()
	except socket.error:
		print("[!] Couldn't connect to server")
		exit()
	
def main():
	target = input("Enter target IP: ")
	start_port = int(input("Start port: "))
	end_port = int(input("End port: "))
	
	threads = []
	print(f"\nScanning {target} from port {start_port} to {end_port}...\n")
	with ThreadPoolExecutor(max_workers=500) as executor:
		for port in range(start_port, end_port + 1):
			executor.submit(start_port, target, port)
			t = threading.Thread(target=scan, args=(target, port))
			t.start()
			threads.append(t)

		for t in threads:
			t.join()
					
if __name__ == '__main__':
	main()
