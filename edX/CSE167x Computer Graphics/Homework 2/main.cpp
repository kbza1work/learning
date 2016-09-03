/******************************************************************************/
/* This is the program skeleton for homework 2 in CSE167x by Ravi Ramamoorthi */
/* Extends HW 1 to deal with shading, more transforms and multiple objects    */
/******************************************************************************/

// You shouldn't have to edit this file at all!

#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <deque>
#include <stack>
#include <GL/glut.h>
#include "shaders.h"
#include "Transform.h"
#include <FreeImage.h>
#include "UCSD/grader.h"
#include "printError.h"

using namespace std;

// Main variables in the program.
#define MAINPROGRAM
#include "variables.h"
#include "readfile.h" // prototypes for readfile.cpp
void display(void);  // prototype for display function.

Grader grader;
bool allowGrader = false;

void reshape(int width, int height)
{
  w = width;
  h = height;

  float aspect = w / (float) h, zNear = 0.1, zFar = 99.0;

  // I am changing the projection stuff to be consistent with lookAt
  if (useGlu) {
    projectionMatrix = glm::perspective(fovy,aspect,zNear,zFar);
  } else {
    projectionMatrix = Transform::perspective(fovy,aspect,zNear,zFar);
  }

  glViewport(0, 0, w, h);
}

void saveScreenshot(string fname)
{
  int pix = w * h;
  BYTE pixels[3*pix];
  glReadBuffer(GL_FRONT);
  glReadPixels(0,0,w,h,GL_BGR,GL_UNSIGNED_BYTE, pixels);

  FIBITMAP *img = FreeImage_ConvertFromRawBits(pixels, w, h, w * 3, 24, 0xFF0000, 0x00FF00, 0x0000FF, false);

  std::cout << "Saving screenshot: " << fname << "\n";

  FreeImage_Save(FIF_PNG, img, fname.c_str(), 0);
}


void printHelp()
{
  std::cout << "\npress 'h' to print this message again.\n"
    << "press '+' or '-' to change the amount of rotation that\noccurs with each arrow press.\n"
    << "press 'i' to run image grader test cases\n"
    << "press 'g' to switch between using glm::lookAt and glm::Perspective or your own LookAt.\n"
    << "press 'r' to reset the transformations.\n"
    << "press 'v' 't' 's' to do view [default], translate, scale.\n"
    << "press ESC to quit.\n";
}


void keyboard(unsigned char key, int x, int y)
{
  switch(key) {
    case '+':
      amount++;
      std::cout << "amount set to " << amount << "\n";
      break;
    case '-':
      amount--;
      std::cout << "amount set to " << amount << "\n";
      break;
    case 'i':
      if(useGlu) {
        std::cout << "Please disable glm::LookAt by pressing 'g'"
          << " before running tests\n";
      }
      else if(!allowGrader) {
        std::cout << "Error: no input file specified for grader\n";
      } else {
        std::cout << "Running tests...\n";
        grader.runTests();
        std::cout << "Done! [ESC to quit]\n";
      }
      break;
    case 'g':
      useGlu = !useGlu;
      reshape(w,h);
      std::cout << "Using glm::LookAt and glm::Perspective set to: "
        << (useGlu ? " true " : " false ") << "\n";
      break;
    case 'h':
      printHelp();
      break;
    case 27:  // Escape to quit
      exit(0);
      break;
    case 'r': // reset eye and up vectors, scale and translate.
      eye = eyeinit;
      up = upinit;
      amount = amountinit;
      transop = view;
      sx = sy = 1.0;
      tx = ty = 0.0;
      break;
    case 'v':
      transop = view;
      std::cout << "Operation is set to View\n";
      break;
    case 't':
      transop = translate;
      std::cout << "Operation is set to Translate\n";
      break;
    case 's':
      transop = scale;
      std::cout << "Operation is set to Scale\n";
      break;
  }
  glutPostRedisplay();
}

//  You will need to enter code for the arrow keys
//  When an arrow key is pressed, it will call your transform functions
void specialKey(int key, int x, int y)
{
  switch(key) {
    case 100: //left
      if (transop == view) Transform::left(amount, eye,  up);
      else if (transop == scale) sx -= amount * 0.01;
      else if (transop == translate) tx -= amount * 0.01;
      break;
    case 101: //up
      if (transop == view) Transform::up(amount,  eye,  up);
      else if (transop == scale) sy += amount * 0.01;
      else if (transop == translate) ty += amount * 0.01;
      break;
    case 102: //right
      if (transop == view) Transform::left(-amount, eye,  up);
      else if (transop == scale) sx += amount * 0.01;
      else if (transop == translate) tx += amount * 0.01;
      break;
    case 103: //down
      if (transop == view) Transform::up(-amount,  eye,  up);
      else if (transop == scale) sy -= amount * 0.01;
      else if (transop == translate) ty -= amount * 0.01;
      break;
  }
  glutPostRedisplay();
}

void initShaders()
{
  vertexshader = initshaders(GL_VERTEX_SHADER, "shaders/light.vert.glsl");
  fragmentshader = initshaders(GL_FRAGMENT_SHADER, "shaders/light.frag.glsl");
  shaderprogram = initprogram(vertexshader, fragmentshader);
  enablelighting = glGetUniformLocation(shaderprogram,"enablelighting");
  printOpenGLError();
  lightpos = glGetUniformLocation(shaderprogram,"lightposn");
  printOpenGLError();
  lightcol = glGetUniformLocation(shaderprogram,"lightColor");
  printOpenGLError();
  numusedcol = glGetUniformLocation(shaderprogram,"numused");
  printOpenGLError();
  ambientcol = glGetUniformLocation(shaderprogram,"ambient");
  printOpenGLError();
  diffusecol = glGetUniformLocation(shaderprogram,"diffuse");
  printOpenGLError();
  specularcol = glGetUniformLocation(shaderprogram,"specular");
  printOpenGLError();
  emissioncol = glGetUniformLocation(shaderprogram,"emission");
  printOpenGLError();
  shininesscol = glGetUniformLocation(shaderprogram,"shininess");
  printOpenGLError();
  modelViewProjectionMatrixcol =
    glGetUniformLocation(shaderprogram, "modelViewProjectionMatrix");
  printOpenGLError();
  modelViewMatrixcol = glGetUniformLocation(shaderprogram, "modelViewMatrix");
  printOpenGLError();
  normalMatrixcol = glGetUniformLocation(shaderprogram, "normalMatrix");
  printOpenGLError();
}

int main(int argc, char* argv[])
{
  if (argc < 2) {
    cerr << "Usage: transforms scenefile [grader input (optional)]\n";
    exit(-1);
  }

  FreeImage_Initialise();
  glutInit(&argc, argv);
  glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGBA | GLUT_DEPTH);
  glutCreateWindow("HW2: Scene Viewer");
  initShaders();
  readfile(argv[1]);
  glutDisplayFunc(display);
  glutSpecialFunc(specialKey);
  glutKeyboardFunc(keyboard);
  glutReshapeFunc(reshape);
  glutReshapeWindow(w, h);

  if (argc > 2) {
    allowGrader = true;
    stringstream tcid;
    tcid << argv[1] << "." << argv[2];
    grader.init(tcid.str());
    grader.loadCommands(argv[2]);
    grader.bindDisplayFunc(display);
    grader.bindSpecialFunc(specialKey);
    grader.bindKeyboardFunc(keyboard);
    grader.bindScreenshotFunc(saveScreenshot);
  }

  printHelp();
  glutMainLoop();
  FreeImage_DeInitialise();
  return 0;
}
