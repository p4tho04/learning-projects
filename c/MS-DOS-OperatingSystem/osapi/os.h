/* os.h */
#pragma once
#define bool int8
#define true 1
#define false 0

typedef unsigned char int8;
typedef unsigned short int16;
typedef unsigned int int32;
typedef unsigned long long int int64;
typedef int8 fd;
typedef int8 error;

#define $1 (int8 *)
#define $2 (int16)
#define $4 (int32)
#define $8 (int64)
#define $c (char *)
#define $i (int)

#define ErrNoErr 0
#define ErrInit 1
#define ErrIO 4
#define ErrBadFD 8

#define packed __attribute__((packed))
#define public __attribute__((visibility("default")))
#define internal __attribute__((visibility("hidden")))
#define private static

#define reterr(x) do {\
    errnumber = (x);      \
    return 0;         \
} while (false)
#define alloc(x)    malloc(x)
#define destroy(x)  free(x)

#ifdef Library
    public bool initialized = false;
    public error errnumber;
#else
    extern public bool initialized;
    extern public error errnumber;
#endif


public bool load(fd, int8); /* Write 1 char */
public int8 store(fd); /* Read 1 char */
public void init(void);
public void dinit(void);