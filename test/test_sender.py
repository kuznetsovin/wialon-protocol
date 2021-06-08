#!/usr/bin/env python3
import socket
from time import sleep


client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('localhost', 5555))

BUFF = 2048

packets = [
    b"#L#1;1\r\n",
    b"#SD#280421;055220;5355.09260;N;02732.40990;E;0;0;300;7\r\n",
    b"#D#280421;055429;5355.09260;N;02732.40990;E;0;0;300;7;22;5;0;;NA;test1:1:1,var:2:4.5,texttest:3:1\r\n",
    b"#SD#280421;055441;5355.09260;N;02732.40990;E;0;0;300;7\r\n",
    b"#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n",
    b"#D#280421;055455;5355.09260;N;02732.40990;E;60;0;300;7;22;5;0;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n",
    b"#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n",
    b"#ASD#1\r",
]

for rec in packets:
    print("send: {}".format(rec))
    client.send(rec)
    rec_package = client.recv(BUFF)
    print("received: {}".format(rec_package.hex()))
    sleep(1)

client.close()
