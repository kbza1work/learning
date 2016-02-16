#ifndef __RENDERER_H__
#define __RENDERER_H__

#include "lib/tinyrenderer/geometry.h"
#include "lib/tinyrenderer/tgaimage.h"

void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color);
void line(Vec2f v0, Vec2f v1, TGAImage &image, TGAColor color);
void triangle(Vec2i *pts, TGAImage &image, TGAColor color);
Vec3i crossProduct(const Vec3i &u, const Vec3i &v);
Vec3f crossProduct(const Vec3f &u, const Vec3f &v);

#endif //__RENDERER_H__

