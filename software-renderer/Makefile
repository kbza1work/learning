SYSCONF_LINK = g++
CPPFLAGS     = -std=c++11
LDFLAGS      =
LIBS         = -lm

DESTDIR = build/
TARGET  = $(DESTDIR)main

TINYRENDERER_DIR = lib/tinyrenderer/


TINYRENDERER_SOURCES := $(filter-out $(TINYRENDERER_DIR)main.cpp, $(filter-out $(TINYRENDERER_DIR)our_gl.cpp, $(wildcard $(TINYRENDERER_DIR)*.cpp)))
TINYRENDERER_OBJECTS := $(patsubst %.cpp,%.o,$(TINYRENDERER_SOURCES))
OBJECTS := $(patsubst %.cpp,%.o,$(wildcard *.cpp)) $(TINYRENDERER_OBJECTS)

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(SYSCONF_LINK) -Wall $(LDFLAGS) -o $(TARGET) $(OBJECTS) $(LIBS)

$(OBJECTS): %.o: %.cpp $(TINYRENDERER_SOURCES)
	$(SYSCONF_LINK) -Wall $(CPPFLAGS) -c $(CFLAGS) $< -o $@

clean:
	-rm -f $(OBJECTS)
	-rm -f $(TARGET)
	-rm -f *.tga

