CC = gcc

CFLAGS = -g -DGL_GLEXT_PROTOTYPES 
INCFLAGS = -I./glm-0.9.2.7 -I./include/ -I/usr/X11R6/include -I/sw/include \
		-I/usr/sww/include -I/usr/sww/pkg/Mesa/include
LDFLAGS = -L/usr/X11R6/lib -L/sw/lib -L/usr/sww/lib \
		-L/usr/sww/bin -L/usr/sww/pkg/Mesa/lib -lglut -lGLEW -lGLU -lGL -lX11 -lfreeimage -lm

RM = /bin/rm -f 
all: presentation

presentation: main.o util.o
	$(CC) $(CFLAGS) -o presentation main.o util.o $(INCFLAGS) $(LDFLAGS) 
main.o: main.c
	$(CC) $(CFLAGS) $(INCFLAGS) -c main.c
util.o: util.c
	$(CC) $(CFLAGS) $(INCFLAGS) -c util.c
clean: 
	$(RM) *.o presentation unitTests
