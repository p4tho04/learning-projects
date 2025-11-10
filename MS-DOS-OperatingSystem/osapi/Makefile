# -fPIC moved to flags from soflags
flags=-O2 -Wall -std=c23 -I. -fPIC
soflags=-ldl -shared
ldflags=

.PHONY: clean 

all: clean osapi.so

osapi.so: osapi.o disk.o
	cc $(flags) $(CLAGS) $(soflags) $^ -o $@

# added -fPIC for both object files
osapi.o: osapi.c osapi.h os.h
	cc $(flags) $(CFLAGS) -c $<

disk.o: disk.c disk.h
	cc $(flags) $(CFLAGS) -c $<

clean:
	rm -f *.so *.o