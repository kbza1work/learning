#include <cmath>
#include <cstring>
#include <iostream>
#include <vector>

#include "lib/tinyrenderer/geometry.h"
#include "lib/tinyrenderer/model.h"
#include "lib/tinyrenderer/tgaimage.h"

const int width = 800;
const int height = 800;
const TGAColor white = TGAColor(255, 255, 255, 255);
const TGAColor red   = TGAColor(255, 0,   0,   255);

void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
	bool steep = false;
	if (std::abs(x0-x1)<std::abs(y0-y1)) {
		std::swap(x0, y0);
		std::swap(x1, y1);
		steep = true;
	}
	if (x0>x1) {
		std::swap(x0, x1);
		std::swap(y0, y1);
	}

	for (int x=x0; x<=x1; x++) {
		float t = (x-x0)/(float)(x1-x0);
		int y = y0*(1.-t) + y1*t;
		if (steep) {
			image.set(y, x, color);
		} else {
			image.set(x, y, color);
		}
	}
}

TGAImage testImage() {
	TGAImage image(width, height, TGAImage::RGB);

	const int centerX = width/2;
	const int centerY = height/2;
	const int blockWidth = 700;
	for(int x = centerX - blockWidth/2; x <= centerX + blockWidth/2; x++) {
		for(int y = centerY - blockWidth/2; y <= centerY + blockWidth/2; y++) {
			const TGAColor color = TGAColor(x % 256, y % 256, (x * y) % 256, 255);
			if(!image.set(x, y, color)) {
				std::cout << "failed to set pixel at (" << x << "," << y << ")"
					<< std::endl;
			}
		}
	}
	return image;
}

TGAImage wireframeImage(Model *model) {
	TGAImage image(width, height, TGAImage::RGB);
	for (int i=0; i<model->nfaces(); i++) {
		std::vector<int> face = model->face(i);
		for (int j=0; j<3; j++) {
			Vec3f v0 = model->vert(face[j]);
			Vec3f v1 = model->vert(face[(j+1)%3]);
			int x0 = (v0.x+1.)*width/2.;
			int y0 = (v0.y+1.)*height/2.;
			int x1 = (v1.x+1.)*width/2.;
			int y1 = (v1.y+1.)*height/2.;
			line(x0, y0, x1, y1, image, white);
		}
	}
	return image;
}

int main(int argc, char** argv) {
    if(argc > 1 && (strcoll(argv[1], "-h") == 0 || strcoll(argv[1], "--help") == 0)) {
        std::cerr << "Usage: " << argv[0] << " <model to render>" << std::endl;
        return 1;
    }

	TGAImage finalImage;

	if(argc < 2) {
		std::cout << "generating test image..." << std::endl;
		finalImage = testImage();
	} else {
		char *modelFilename = argv[1];
		std::cout << "generating wireframe from model at " << modelFilename <<
			"..." << std::endl;
		Model *model = new Model(modelFilename);
		finalImage = wireframeImage(model);
		delete model;
	}

	// I want to have the origin at the left bottom corner of the image
	finalImage.flip_vertically();
	finalImage.write_tga_file("output.tga");
	return 0;
}
