#!/usr/bin/env python3
import socket
from time import sleep


client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('localhost', 5555))

BUFF = 2048

with open("test_packet.txt", "rb") as f:
    for rec in f.readlines():
        print("send: {}".format(rec))
        client.send(rec)
        rec_package = client.recv(BUFF)
        print("received: {}".format(rec_package.hex()))
        sleep(1)

client.close()
