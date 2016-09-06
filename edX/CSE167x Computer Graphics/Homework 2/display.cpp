/******************************************************************************/
/* This is the program skeleton for homework 2 in CSE167x by Ravi Ramamoorthi */
/* Extends HW 1 to deal with shading, more transforms and multiple objects    */
/******************************************************************************/

// This file is display.cpp.  It includes the skeleton for the display routine

// Basic includes to get this file to work.
#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <deque>
#include <stack>
#include <GL/glut.h>
#include <GL/freeglut_ext.h>
#include "Transform.h"
#include "printError.h"

// DEBUG
#include "debug.h"
#include "glm/gtx/string_cast.hpp"

using namespace std ;
#include "variables.h"
#include "readfile.h"

void display() {
  glClearColor(0, 0, 1, 0);
  printOpenGLError();
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  printOpenGLError();

  // Either use the built-in lookAt function or the lookAt implemented by the user.
  mat4 viewMatrix;
  if (useGlu) {
    viewMatrix = glm::lookAt(eye,center,up);
  } else {
    viewMatrix = Transform::lookAt(eye,center,up);
  }

  // Lights are transformed by current modelview matrix.
  // The shader can't do this globally.
  // So we need to do so manually.
  if (numused) {
    glUniform1i(enablelighting,true);
    printOpenGLError();

    GLfloat transformedLightPositions[4 * numLights];
    for(int lightCount = 0; lightCount < numused; lightCount++) {
      const size_t offset = 4 * lightCount;
      vec4 thisLightPosition = vec4(
        lightposn[offset],
        lightposn[offset + 1],
        lightposn[offset + 2],
        lightposn[offset + 3]
      );

      thisLightPosition = viewMatrix * thisLightPosition;

      transformedLightPositions[offset] = thisLightPosition.x;
      transformedLightPositions[offset + 1] = thisLightPosition.y;
      transformedLightPositions[offset + 2] = thisLightPosition.z;
      transformedLightPositions[offset + 3] = thisLightPosition.w;
    }

    glUniform4fv(lightpos, 4 * numused, transformedLightPositions);
    printOpenGLError();
    glUniform4fv(lightcol, 4 * numused, lightcolor);
    printOpenGLError();
    glUniform1i(numusedcol, numused);
    printOpenGLError();

// DEBUG
/* std::cout << "transformed light positions = ["; */
/* for(int i = 0; i < (4 * numused); i++) { */
/*   std::cout << transformedLightPositions[i]; */
/*   if(i < ((4 * numused) - 1)) { */
/*     std::cout << ", "; */
/*   } */
/* } */
/* std::cout << "]" << std::endl; */
/* std::cout << "light colors = ["; */
/* for(int i = 0; i < (4 * numused); i++) { */
/*   std::cout << lightcolor[i]; */
/*   if(i < ((4 * numused) - 1)) { */
/*     std::cout << ", "; */
/*   } */
/* } */
/* std::cout << "]" << std::endl; */
/* std::cout << "number of active lights = " << numused << std::endl; */
  } else {
    glUniform1i(enablelighting,false);
    printOpenGLError();
  }

  // Transformations for objects, involving translation and scaling
  mat4 sc(1.0) , tr(1.0), transf(1.0);
  sc = Transform::scale(sx,sy,1.0);
  tr = Transform::translate(tx,ty,0.0);

  for (int i = 0 ; i < numobjects ; i++) {
    object* obj = &(objects[i]); // Grabs an object struct.

    glUniform4fv(ambientcol, 1, obj->ambient);
    printOpenGLError();
    glUniform4fv(diffusecol, 1, obj->diffuse);
    printOpenGLError();
    glUniform4fv(specularcol, 1, obj->specular);
    printOpenGLError();
    glUniform4fv(emissioncol, 1, obj->emission);
    printOpenGLError();
    glUniform1f(shininesscol, obj->shininess);
    printOpenGLError();

    const mat4 modelViewMatrix = viewMatrix * tr * sc * obj->transform;
    const mat4 normalMatrix = glm::transpose(glm::inverse(modelViewMatrix));
    const mat4 mvp = projectionMatrix * modelViewMatrix;

    glUniformMatrix4fv(modelViewMatrixcol, 1, GL_FALSE, &modelViewMatrix[0][0]);
    printOpenGLError();
    glUniformMatrix4fv(normalMatrixcol, 1, GL_FALSE, &normalMatrix[0][0]);
    printOpenGLError();
    glUniformMatrix4fv(modelViewProjectionMatrixcol, 1, GL_FALSE, &mvp[0][0]);
    printOpenGLError();

// DEBUG
/* std::cout << "eye (world coordinates) = " << glm::to_string(eye) << std::endl; */
/* std::cout << "up (world coordinates) = " << glm::to_string(up) << std::endl; */
/* DEBUG(obj->transform, "obj->transform"); */
/* DEBUG(tr, "tr"); */
/* DEBUG(viewMatrix, "view"); */
/* DEBUG(modelViewMatrix, "modelview"); */
/* DEBUG(projectionMatrix, "projection"); */
/* DEBUG(mvp, "final MVP"); */
/* std::cout << "diffuse = ["; */
/* for(int i = 0; i < 4; i++) { */
/*   std::cout << obj->diffuse[i]; */
/*   if(i < 3) { */
/*     std::cout << ", "; */
/*   } */
/* } */
/* std::cout << "]" << std::endl; */
/* std::cout << "specular = ["; */
/* for(int i = 0; i < 4; i++) { */
/*   std::cout << obj->specular[i]; */
/*   if(i < 3) { */
/*     std::cout << ", "; */
/*   } */
/* } */
/* std::cout << "]" << std::endl; */
/* std::cout << "emission = ["; */
/* for(int i = 0; i < 4; i++) { */
/*   std::cout << obj->emission[i]; */
/*   if(i < 3) { */
/*     std::cout << ", "; */
/*   } */
/* } */
/* std::cout << "]" << std::endl; */
/* std::cout << "shininess = " << obj->shininess << std::endl; */
/* std::cout << std::endl; */

    // Actually draw the object
    // We provide the actual glut drawing functions for you.
    // Remember that obj->type is notation for accessing struct fields
    if (obj->type == cube) {
      glutSolidCube(obj->size);
      printOpenGLError();
    } else if (obj->type == sphere) {
      const GLint tessel = 20;
      glutSolidSphere(obj->size, tessel, tessel);
      printOpenGLError();
    } else if (obj->type == cylinder) {
      const GLint tessel = 20;
      glutSolidCylinder(obj->size, 1.0, tessel, tessel);
      printOpenGLError();
    } else if (obj->type == teapot) {
      glutSolidTeapot(obj->size);
      printOpenGLError();
    }

  }

  glutSwapBuffers();
}
