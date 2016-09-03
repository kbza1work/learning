#ifndef PRINT_ERROR_H
#define PRINT_ERROR_H

#define printOpenGLError() printOglError(__FILE__, __LINE__)

int printOglError(char const *file, int line);

#endif /* PRINT_ERROR_H */
