#!/usr/bin/env python3
import socket

HOST = ''
PORT = 5000
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen(1)
    print("start mock server")

    conn, addr = s.accept()
    with conn:
        print('Connected by', addr)
        counter = 0
        while True:
            data = conn.recv(1024)
            if not data:
                break
            # Печать полученных сообщений
            print(data)
            if counter == 0:
                # Сообщение упешной авторизации
                conn.sendall(b"#AL#1\r\n")
