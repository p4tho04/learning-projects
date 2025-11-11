flags := -O2 -Wall -I. -Wno-attributes

all: clean tcpserver

tcpserver: tcpserver.o
	gcc $(flags) tcpserver.o -o tcpserver

tcpserver.o: tcpserver.c
	gcc $(flags) -c tcpserver.c

clean:
	rm -f *.o tcpserver