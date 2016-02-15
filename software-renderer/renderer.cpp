#include <cstdlib>
#include <deque>

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

void line(Vec3f v0, Vec3f v1, TGAImage &image, TGAColor color) {
	line(round(v0.x), round(v0.y), round(v1.x), round(v1.y), image, color);
}

void triangle(Vec3f v0, Vec3f v1, Vec3f v2, TGAImage &image, TGAColor color) {
	// compute a line between v1 and v2, then iterate over points along the
	// line, drawing lines from v0 to the current point on the line between v1
	// and v2 to create a filled triangle
	std::deque<Vec3f> longestSide;
	Vec3f loneVertex;
	longestSide.push_back(v0);
	longestSide.push_back(v1);
	longestSide.push_back(v2);
	const float s0 = distance(longestSide[0], longestSide[1]);
	const float s1 = distance(longestSide[0], longestSide[2]);
	const float s2 = distance(longestSide[1], longestSide[2]);
	if(s0 > s1) {
		if(s0 > s2) {
			loneVertex = v2;
			longestSide.erase(longestSide.begin()+2);
		} else {
			loneVertex = v0;
			longestSide.erase(longestSide.begin());
		}
	} else {
		if(s1 > s2) {
			loneVertex = v1;
			longestSide.erase(longestSide.begin()+1);
		} else {
			loneVertex = v0;
			longestSide.erase(longestSide.begin());
		}
	}

	for(auto const& point: lineSeries(longestSide[0], longestSide[1])) {
		line(loneVertex, point, image, color);
	}
}


// private

std::vector<Vec3f> lineSeries(Vec3f v0, Vec3f v1) {
	std::vector<Vec3f> result;

	// a type to store the index of the steepest dimension in the line between
	// the two vectors
	enum steepest_dim_t {X = 0, Y = 1, Z = 1};

	steepest_dim_t steepest;
	{
		const int x0 = v0.x;
		const int y0 = v0.y;
		const int z0 = v0.z;
		const int x1 = v1.x;
		const int y1 = v1.y;
		const int z1 = v1.z;

		if(std::abs(x0-x1) > std::abs(y0-y1)) {
			if(std::abs(x0-x1) > std::abs(z0-z1)) {
				steepest = X;
			} else {
				steepest = Z;
			}
		} else {
			if(std::abs(z0-z1) > std::abs(y0-y1)) {
				steepest = Z;
			} else {
				steepest = Y;
			}
		}
	}

	if(v0[steepest] > v1[steepest]) {
		std::swap(v0, v1);
	}
	for(int i=v0[steepest]; i<=v1[steepest]; i++) {
		float t = (i-v0[steepest])/(float)(v1[steepest]-v0[steepest]);
		Vec3f nextPoint;
		for(int dim = 0; dim < 3; dim++) {
			if(dim == steepest) {
				nextPoint[dim] = i;
			} else {
				nextPoint[dim] = v0[dim]*(1.0-t) + v1[dim]*t;
			}
		}
		result.push_back(nextPoint);
	}

	return result;
}

float distance(Vec3f v0, Vec3f v1) {
	return sqrt(pow(v1.x - v0.x, 2) + pow(v1.y - v0.y, 2));
}
