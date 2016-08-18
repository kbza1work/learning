CC = g++
ifeq ($(shell sw_vers 2>/dev/null | grep Mac | awk '{ print $$2}'),Mac)
CFLAGS = -g -DGL_GLEXT_PROTOTYPES -DOSX
INCFLAGS = -I./glm-0.9.2.7 -I/usr/X11/include -I./include/
LDFLAGS = -framework GLUT -framework OpenGL -L./osxlib/ \
		-L"/System/Library/Frameworks/OpenGL.framework/Libraries" \
		-lGL -lGLU -lm -lstdc++ -lfreeimage
else
CFLAGS = -g -DGL_GLEXT_PROTOTYPES
INCFLAGS = -I./glm-0.9.2.7 -I./include/ -I/usr/X11R6/include -I/sw/include \
		-I/usr/sww/include -I/usr/sww/pkg/Mesa/include
LDFLAGS = -L/usr/X11R6/lib -L/sw/lib -L/usr/sww/lib \
		-L/usr/sww/bin -L/usr/sww/pkg/Mesa/lib -lglut -lGLU -lGL -lX11 -lfreeimage
endif

RM = /bin/rm -f
all: transform
tests: unitTests

unitTests: Transform.o Transform.h types.h unitTests.o TransformTests.o readfile.o readfile.h readfileTests.o
	$(CC) $(CFLAGS) -o unitTests Transform.o unitTests.o TransformTests.o readfile.o readfileTests.o $(INCFLAGS) $(LDFLAGS)

transform: main.o types.h shaders.o Transform.o readfile.o display.o variables.h readfile.h shaders.h Transform.h grader.o UCSD/grader.h
	$(CC) $(CFLAGS) -o transforms shaders.o main.o Transform.o readfile.o display.o grader.o $(INCFLAGS) $(LDFLAGS)

main.o: main.cpp shaders.h Transform.h variables.h types.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c main.cpp
shaders.o: shaders.cpp shaders.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c shaders.cpp
readfile.o: readfile.cpp readfile.h variables.h types.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c readfile.cpp
display.o: display.cpp variables.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c display.cpp
Transform.o: Transform.cpp Transform.h types.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c Transform.cpp
grader.o: UCSD/grader.cpp UCSD/grader.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c UCSD/grader.cpp

unitTests.o: unitTests.cpp types.h variables.h types.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c unitTests.cpp
TransformTests.o: TransformTests.cpp types.h Transform.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c TransformTests.cpp
readfileTests.o: readfileTests.cpp types.h readfile.h
	$(CC) $(CFLAGS) $(INCFLAGS) -c readfileTests.cpp

clean:
	$(RM) *.o transforms unitTests *.png

