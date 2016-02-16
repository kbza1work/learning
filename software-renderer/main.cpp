#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

#include "lib/tinyrenderer/geometry.h"
#include "lib/tinyrenderer/model.h"
#include "lib/tinyrenderer/tgaimage.h"

#include "renderer.h"

const int width = 800;
const int height = 800;

int screenX(float x) {
	return round((x + 1.0) * width/2.0);
}
int screenX(int x) {
	return round((x + 1) * width/2);
}

int screenY(float y) {
	return round((y + 1.0) * height/2.0);
}
int screenY(int y) {
	return round((y + 1) * height/2);
}

// convert a vector with axes in the range [-1, 1] to screen space axes with
// range [0, width]
Vec2i screen(Vec3f vector) {
	return Vec2i(screenX(vector.x), screenY(vector.y));
}
Vec2i screen(Vec3i vector) {
	return Vec2i(screenX(vector.x), screenY(vector.y));
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
	const TGAColor white = TGAColor(255, 255, 255, 255);
	for (int i=0; i<model->nfaces(); i++) {
		std::vector<int> face = model->face(i);
		for (int j=0; j<3; j++) {
			Vec2i v0 = screen(model->vert(face[j]));
			Vec2i v1 = screen(model->vert(face[(j+1)%3]));
			line(v0, v1, image, white);
		}
	}
	return image;
}

TGAImage filledImage(Model *model) {
	TGAImage image(width, height, TGAImage::RGB);
	const Vec3f light_dir(0,0,-1);
	for (int i=0; i<model->nfaces(); i++) {
		std::vector<int> face = model->face(i);
		Vec2i screen_coords[3];
		Vec3f world_coords[3];
		for (int j=0; j<3; j++) {
			Vec3f v = model->vert(face[j]);
			screen_coords[j] = Vec2i((v.x+1.)*width/2., (v.y+1.)*height/2.);
			world_coords[j]  = v;
		}
		Vec3f n = crossProduct((world_coords[2]-world_coords[0]), (world_coords[1]-world_coords[0]));
		n.normalize();
		float intensity = n*light_dir;
		if (intensity>0) {
			auto color = TGAColor(intensity*255, intensity*255, intensity*255, 255);
			triangle(screen_coords, image, color); 
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
		std::cout << "generating filled image from model at " << modelFilename <<
			"..." << std::endl;
		Model *model = new Model(modelFilename);
		finalImage = filledImage(model);
		delete model;
	}

	// I want to have the origin at the left bottom corner of the image
	finalImage.flip_vertically();
	finalImage.write_tga_file("output.tga");
	return 0;
}
