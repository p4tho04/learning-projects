/* tcpserver.h */
#define _GNU_SOURCE
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>
#include <errno.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <string.h>

typedef unsigned char int8;
typedef unsigned short int int16;
typedef unsigned int int32;
typedef unsigned long long int int64;

#define public  __attribute__((visibility("default")))
#define private __attribute__((visibility("hidden"))) static
#define init    __attribute__((constructor))

#define $c (char *)
#define $i (int)
#define $1 (int8 *)
#define $2 (int16)
#define $4 (int32)
#define $8 (int64)

#define reterr(x)   do { \
    server *_srv; \
    int16 _sz, _len; \
    \
    _sz = sizeof(struct s_server) \
        + (_len = $2 strlen($c (x))) \
        + 1; \
    _srv = (server *)malloc($i _sz); \
    assert(_srv); \
    memset($c _srv, 0, $i _sz); \
    _srv->status = Errstatus; \
    memcpy($c _srv->errmsg, $c (x), $i _len); \
    \
    return _srv; \
} while (false)

enum {
    Errstatus,
    Okstatus
};

struct s_connection {
    int32 fd;
    int8 host[16];
    int16 port;
    int (*send)(char*, int);
    int (*recv)(char*, int);
};
typedef struct s_connection connection;

typedef void (*callback)(connection*);

struct s_server {
    int32 fd;
    int8 status;
    int16 port;
    bool (*ok)(void);
    char *(*err)(void);
    int (*listen)(void);
    callback cb;
    int8 errmsg[];
};
typedef struct s_server server;

int main(void);
private void handler_(connection*);
private init void start(void);
public server *tcpserver(int16, callback);
private bool okfunction(void);
private char *errfunction(void);
private int listenfunction(void);
private int recvfunction(char*,int);
private int sendfunction(char*,int);

