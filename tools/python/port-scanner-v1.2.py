import socket
import sys

def parse_args():
	ip = '127.0.0.1'
	start_port = 1
	end_port = 65535

	if len(sys.argv) >= 2:
		ip_arg = sys.argv[1]
		ip = '127.0.0.1' if ip_arg == 'localhost' else ip_arg

	if len(sys.argv) == 4:
		try:
			start_port = int(sys.argv[2])
			end_port = int(sys.argv[3])
			if start_port > end_port or not (1 <= start_port <= 65535 and 1 <= end_port <= 65535):
				raise ValueError
		except ValueError:
			print("Invalid port range. Port must be between 1 and 65535.")
			sys.exit(1)

	return ip, start_port, end_port

def scan_port(ip, port):
	with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
		s.settimeout(0.1)
		result = s.connect_ex((ip, port))
		return result == 0

def main():
	ip, start_port, end_port = parse_args()
	print(f"[*] Scanning {ip} from port {start_port} to {end_port}...")

	for port in range(start_port, end_port + 1):
		if scan_port(ip, port):
			print(f"[+] Port {port} is OPEN...")

	print("[+] Scan Completed...")

if __name__ == '__main__':
	main()
