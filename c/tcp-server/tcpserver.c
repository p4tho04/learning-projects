/* tcpserver.c */
#include "tcpserver.h"

public server *globalsrv;
public connection *globalconn;

#define handler()   &handler_
private void handler_(connection *conn) {
    char buf[128];

    memset(buf, 0, 128);
    conn->send("Welcome to Server\n", 19);
    conn->recv(buf, 127);
    printf("Received: '%s'\n", buf);
    return;
}

private init void start() {
    globalsrv = (server *)0;
    globalconn = (connection *)0;
    
    return;
}

public server *tcpserver(int16 port, callback cb) {
    signed int ret;
    int32 s;
    struct sockaddr_in sock;
    server *srv;
    int16 size;

    if (globalsrv)
        reterr($1 "tcp server already initialized");

    ret = socket(AF_INET, SOCK_STREAM, 0);
    if (ret < 0)
        reterr($1 "Failed to initialize socket");
    else
        s = $4 ret;
    
    sock.sin_family = AF_INET;
    sock.sin_port = htons($i port);
    sock.sin_addr.s_addr = INADDR_ANY;

    ret = bind($i s, (struct sockaddr *)&sock, sizeof(struct sockaddr));
    if (ret) {
        close($i s);
        reterr($1 "Unable to bind to port");
    }

    ret = listen($i s, 10);
    if (ret) {
        close($i s);
        reterr($1 "Unable to listen to port");
    }

    size = sizeof(struct s_server);
    srv = (server *)malloc($i size);
    assert(srv);
    memset($c srv, 0, $i size);

    srv->fd = s;
    srv->port = port;
    srv->status = Okstatus;
    srv->ok = &okfunction;
    srv->err = &errfunction;
    srv->listen = &listenfunction;
    srv->cb = cb;
    globalsrv = srv;

    return srv;
}

private bool okfunction() {
    if (!globalsrv)
        return false;

    return (globalsrv->status == Okstatus) ?
        true :
        false;
}

private char *errfunction() {
    if (!globalsrv) 
        return "tcp server not initialized";
    else if (globalsrv->status != Errstatus)
        return "no error";
    else
        return $c globalsrv->errmsg;
}

private int listenfunction() {
    int32 s;
    signed int ret;
    struct sockaddr_in sock;
    socklen_t len;
    socklen_t *lenptr;
    int16 size;
    connection *conn;

    if (!globalsrv)
        return -1;
    else if (globalsrv->status == Errstatus)
        return -1;
    
    // Creates child process and exits parent process, so all func code after here is ran in background
    if (fork())
        exit(0);

    size = sizeof(struct s_connection);
    len = sizeof(struct sockaddr_in);
    lenptr = &len;

    while (true) {
        ret = accept($i globalsrv->fd, (struct sockaddr *)&sock, lenptr);
        if (ret < 0) {
            sleep(2);
            continue;
        }
        else
            s = $4 ret;

        conn = (connection *)malloc($i size);
        if (!conn) {
            close($i s);
            sleep(2);
            continue;
        }
        memset($c conn, 0, $i size);
        conn->fd = s;
        strncpy($c conn->host, inet_ntoa(sock.sin_addr), 15);
        conn->port = $2 htons(sock.sin_port);
        conn->send = &sendfunction;
        conn->recv = &recvfunction;

        if (fork()) {
            if (conn)
                free(conn);
            continue;
        }

        globalconn = conn;
        globalsrv->cb(conn);
        exit(0);
    }
    exit(0);
    return 0;
}

private int recvfunction(char *c, int sz) {
    if (!globalconn || !c || !sz)
        return -1;
    else
        return read($i globalconn->fd, c, sz);
}

private int sendfunction(char *c, int sz) {
    if (!globalconn || !c || !sz)
        return -1;
    else
        return write($i globalconn->fd, c, sz);
}

int main() {
    server *serv;

    serv = tcpserver(8585, handler());
    if (serv && !serv->ok()) {
        printf("Error: %s\n", serv->err());
        return -1;
    }

    printf("Listening on port %d\n", 8585);

    return serv->listen();
}