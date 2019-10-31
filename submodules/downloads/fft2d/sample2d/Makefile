# ---- for GNU gcc ----

CC = gcc

CFLAGS = -Wall

OFLAGS = -O2

# ---- for SUN WS cc ----
#
#CC = cc
#
#CFLAGS = 
#
#OFLAGS = -xO2




all: fft4f2dt fftsg2dt fftsg3dt shrtdctt


fft4f2dt : fft4f2dt.o fft4f2d.o alloc.o
	$(CC) fft4f2dt.o fft4f2d.o alloc.o -lm -o fft4f2dt

fftsg2dt : fftsg2dt.o fftsg2d.o fftsg.o alloc.o
	$(CC) fftsg2dt.o fftsg2d.o fftsg.o alloc.o -lm -o fftsg2dt

fftsg3dt : fftsg3dt.o fftsg3d.o fftsg.o alloc.o
	$(CC) fftsg3dt.o fftsg3d.o fftsg.o alloc.o -lm -o fftsg3dt

shrtdctt : shrtdctt.o shrtdct.o
	$(CC) shrtdctt.o shrtdct.o -lm -o shrtdctt


fft4f2dt.o : fft4f2dt.c
	$(CC) $(CFLAGS) $(OFLAGS) -c fft4f2dt.c -o fft4f2dt.o

fftsg2dt.o : fftsg2dt.c
	$(CC) $(CFLAGS) $(OFLAGS) -c fftsg2dt.c -o fftsg2dt.o

fftsg3dt.o : fftsg3dt.c
	$(CC) $(CFLAGS) $(OFLAGS) -c fftsg3dt.c -o fftsg3dt.o

shrtdctt.o : shrtdctt.c
	$(CC) $(CFLAGS) $(OFLAGS) -c shrtdctt.c -o shrtdctt.o


fft4f2d.o : ../fft4f2d.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../fft4f2d.c -o fft4f2d.o

fftsg2d.o : ../fftsg2d.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../fftsg2d.c -o fftsg2d.o

fftsg3d.o : ../fftsg3d.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../fftsg3d.c -o fftsg3d.o

fftsg.o : ../fftsg.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../fftsg.c -o fftsg.o

alloc.o : ../alloc.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../alloc.c -o alloc.o

shrtdct.o : ../shrtdct.c
	$(CC) $(CFLAGS) $(OFLAGS) -c ../shrtdct.c -o shrtdct.o




clean:
	rm -f *.o

