#include <cstdlib>
#include <vector>

#include "lib/tinyrenderer/geometry.h"
#include "lib/tinyrenderer/tgaimage.h"

#include "renderer.h"


std::vector<Vec3f> lineSeries(Vec3f v0, Vec3f v1);
float distance(Vec3f v0, Vec3f v1);

void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
	bool steep = false;
	if(std::abs(x0-x1)<std::abs(y0-y1)) {
		std::swap(x0, y0);
		std::swap(x1, y1);
		steep = true;
	}
	if(x0>x1) {
		std::swap(x0, x1);
		std::swap(y0, y1);
	}

	for(int x=x0; x<=x1; x++) {
		float t = (x-x0)/(float)(x1-x0);
		int y = y0*(1.-t) + y1*t;
		if (steep) {
			image.set(y, x, color);
		} else {
			image.set(x, y, color);
		}
	}
}

void line(Vec2f v0, Vec2f v1, TGAImage &image, TGAColor color) {
	line(round(v0.x), round(v0.y), round(v1.x), round(v1.y), image, color);
}

void line(Vec2i v0, Vec2i v1, TGAImage &image, TGAColor color) {
	line(v0.x, v0.y, v1.x, v1.y, image, color);
}

Vec3f barycentric(Vec2i *pts, Vec2i P) { 
	Vec3f u = cross(Vec3f(pts[2][0]-pts[0][0], pts[1][0]-pts[0][0], pts[0][0]-P[0]), Vec3f(pts[2][1]-pts[0][1], pts[1][1]-pts[0][1], pts[0][1]-P[1])); 
	// triangle is degenerate, in this case return smth with negative coordinates 
	if (std::abs(u[2])<1) return Vec3f(-1,1,1);
	return Vec3f(1.f-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z); 
} 

void triangle(Vec2i *pts, TGAImage &image, TGAColor color) { 
	Vec2i bboxmin(image.get_width()-1,	image.get_height()-1); 
	Vec2i bboxmax(0, 0); 
	Vec2i clamp(image.get_width()-1, image.get_height()-1); 
	for (int i=0; i<3; i++) { 
		for (int j=0; j<2; j++) { 
			bboxmin[j] = std::max(0,		std::min(bboxmin[j], pts[i][j])); 
			bboxmax[j] = std::min(clamp[j], std::max(bboxmax[j], pts[i][j])); 
		} 
	} 
	Vec2i P; 
	for (P.x=bboxmin.x; P.x<=bboxmax.x; P.x++) { 
		for (P.y=bboxmin.y; P.y<=bboxmax.y; P.y++) { 
			Vec3f bc_screen  = barycentric(pts, P); 
			if (bc_screen.x<0 || bc_screen.y<0 || bc_screen.z<0) continue; 
			image.set(P.x, P.y, color); 
		} 
	} 
} 

Vec3i crossProduct(const Vec3i &u, const Vec3i &v) {
	return Vec3i(u.y*v.z - u.z*v.y, u.z*v.x - u.x*v.z, u.x*v.y - u.y*v.x);
}

Vec3f crossProduct(const Vec3f &u, const Vec3f &v) {
	return Vec3f(u.y*v.z - u.z*v.y, u.z*v.x - u.x*v.z, u.x*v.y - u.y*v.x);
}
