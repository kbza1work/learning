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
#include "Transform.h"
#include "printError.h"

// DEBUG
#include "debug.h"
#include "glm/gtx/string_cast.hpp"

using namespace std ;
#include "variables.h"
#include "readfile.h"

// New helper transformation function to transform vector by modelview
// May be better done using newer glm functionality.
// Provided for your convenience.  Use is optional.
// Some of you may want to use the more modern routines in readfile.cpp
// that can also be used.
void transformvec (const GLfloat input[4], GLfloat output[4])
{
  GLfloat modelview[16]; // in column major order
  glGetFloatv(GL_MODELVIEW_MATRIX, modelview);
  printOpenGLError();

  for (int i = 0 ; i < 4 ; i++) {
    output[i] = 0;
    for (int j = 0 ; j < 4 ; j++) {
      output[i] += modelview[4*j+i] * input[j];
    }
  }
}

void display()
{
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

    glUniform4fv(lightpos, numused, lightposn);
    printOpenGLError();
    glUniform4fv(lightcol, numused, lightcolor);
    printOpenGLError();
    glUniform1i(numusedcol, numused);
    printOpenGLError();
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

    const mat4 modelViewMatrix = viewMatrix * sc * tr * obj->transform;
    const mat4 normalMatrix =
      glm::normalize(glm::transpose(glm::inverse(modelViewMatrix)));
    const mat4 mvp = projectionMatrix * modelViewMatrix;

    glUniformMatrix4fv(modelViewMatrixcol, 1, GL_FALSE, &modelViewMatrix[0][0]);
    printOpenGLError();
    glUniformMatrix4fv(normalMatrixcol, 1, GL_FALSE, &normalMatrix[0][0]);
    printOpenGLError();
    glUniformMatrix4fv(modelViewProjectionMatrixcol, 1, GL_FALSE, &mvp[0][0]);
    printOpenGLError();

// DEBUG
std::cout << "eye = " << glm::to_string(eye) << std::endl;
std::cout << "up = " << glm::to_string(up) << std::endl;
DEBUG(obj->transform, "obj->transform");
DEBUG(tr, "tr");
DEBUG(modelViewMatrix, "modelview/look at");
DEBUG(projectionMatrix, "projection");
DEBUG(mvp, "final MVP");
std::cout << std::endl;

    // Actually draw the object
    // We provide the actual glut drawing functions for you.
    // Remember that obj->type is notation for accessing struct fields
    if (obj->type == cube) {
      glutSolidCube(obj->size);
      printOpenGLError();
    }
    else if (obj->type == sphere) {
      const int tessel = 20;
      glutSolidSphere(obj->size, tessel, tessel);
      printOpenGLError();
    }
    else if (obj->type == teapot) {
      glutSolidTeapot(obj->size);
      printOpenGLError();
    }

  }

  glutSwapBuffers();
}
