#ifndef __RENDERER_H__
#define __RENDERER_H__

#include "lib/tinyrenderer/geometry.h"
#include "lib/tinyrenderer/tgaimage.h"

void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color);
void line(Vec3f v0, Vec3f v1, TGAImage &image, TGAColor color);
void triangle(Vec3f v0, Vec3f v1, Vec3f v2, TGAImage &image, TGAColor color);

#endif //__RENDERER_H__

