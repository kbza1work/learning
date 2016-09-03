#include "printError.h"
#include <cstdio>
#include <GL/gl.h>
#include <GL/glu.h>

//-----------------------------------------------------------------
// Print for OpenGL errors
//
// Returns 1 if an OpenGL error occurred, 0 otherwise.
//
// copied from a Lighthouse 3D tutorial:
// http://www.lighthouse3d.com/cg-topics/error-tracking-in-opengl/
//
int printOglError(char const *file, int line)
{
    GLenum glErr;
    int retCode = 0;

    glErr = glGetError();
    if (glErr != GL_NO_ERROR)
    {
        printf("glError in file %s @ line %d: %s\n",
			     file, line, gluErrorString(glErr));
        retCode = 1;
    }
    return retCode;
}
