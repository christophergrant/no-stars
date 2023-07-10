import socket
import requests
from multiprocessing import Pool

# Range of ports to scan
start_port = 1
end_port = 1024

# Timeout for the socket connection in seconds
timeout = 0.1

def scan_ports(port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.settimeout(timeout)
        try:
            s.connect(('localhost', port))
            try:
                r = requests.get(f'http://localhost:{port}/metrics')
                if r.status_code == 200:
                    return f'Port {port} is open and serving Prometheus metrics'
                else:
                    return f'Port {port} is open'
            except requests.exceptions.RequestException:
                return f'Port {port} is open'
        except (socket.timeout, ConnectionRefusedError):
            return None

if __name__ == "__main__":
    with Pool(20) as p: # Create a multiprocessing Pool
        for result in p.imap_unordered(scan_ports, range(start_port, end_port+1)):
            if result is not None:
                print(result)
