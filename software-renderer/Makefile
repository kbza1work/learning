SYSCONF_LINK = g++
CPPFLAGS     = -Ilib/tinyrenderer
LDFLAGS      = -Ilib/tinyrenderer -Llib/tinyrenderer
LIBS         = -lm -ltgaimage

DESTDIR = build/
TARGET  = $(DESTDIR)main

TINYRENDERER_DIR = lib/tinyrenderer/


OBJECTS := $(patsubst %.cpp,%.o,$(wildcard *.cpp))
TINYRENDERER_LIB := $(TINYRENDERER_DIR)libtgaimage.a

all: $(TARGET)

$(TARGET): $(OBJECTS) $(TINYRENDERER_LIB)
	$(SYSCONF_LINK) -Wall $(LDFLAGS) -o $(TARGET) $(OBJECTS) $(LIBS)

$(OBJECTS): %.o: %.cpp
	$(SYSCONF_LINK) -Wall $(CPPFLAGS) -c $(CFLAGS) $< -o $@

# compile tinyrenderer into a static library
TINYRENDERER_OBJECTS := $(patsubst %.cpp,%.o,$(wildcard $(TINYRENDERER_DIR)*.cpp))

$(TINYRENDERER_LIB): $(TINYRENDERER_OBJECTS)
	ar rvs $(TINYRENDERER_DIR)libtgaimage.a $(TINYRENDERER_DIR)tgaimage.o

$(TINYRENDERER_OBJECTS): $(TINYRENDERER_DIR)tgaimage.cpp
	$(SYSCONF_LINK) -Wall $(CPPFLAGS) -c $(CFLAGS) $< -o $@


clean:
	-rm -f $(OBJECTS)
	-rm -f $(TINYRENDERER_LIB)
	-rm -f $(TARGET)
	-rm -f *.tga

