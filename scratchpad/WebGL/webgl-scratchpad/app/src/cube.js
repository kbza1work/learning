import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

const CUBE_TEXTURE_URL = "assets/textures/glass.gif";

const TEXTURE_SHADERS = [
	"texture-vertex-shader",
	"texture-fragment-shader",
];
const TEXTURE_SHADER_ATTRIBUTES = [
	"aVertexPosition",
	"aTextureCoord",
];
const TEXTURE_SHADER_UNIFORMS = [
	"modelViewMatrix",
	"perspectiveMatrix",
	"uSampler",
	"uAlpha",
];

export default function Cube(gl) {
	this.gl = gl;

	this.shaders = Util.initShaders(
		gl,
		TEXTURE_SHADERS,
		TEXTURE_SHADER_ATTRIBUTES,
		TEXTURE_SHADER_UNIFORMS
	);

	this.texture = Util.initTexture(this.gl, CUBE_TEXTURE_URL);

	this.position = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.position);
	const vertices = [
		// Front face
		-1.0, -1.0,  1.0,
		 1.0, -1.0,  1.0,
		 1.0,  1.0,  1.0,
		-1.0,  1.0,  1.0,

		// Back face
		-1.0, -1.0, -1.0,
		-1.0,  1.0, -1.0,
		 1.0,  1.0, -1.0,
		 1.0, -1.0, -1.0,

		// Top face
		-1.0,  1.0, -1.0,
		-1.0,  1.0,  1.0,
		 1.0,  1.0,  1.0,
		 1.0,  1.0, -1.0,

		// Bottom face
		-1.0, -1.0, -1.0,
		 1.0, -1.0, -1.0,
		 1.0, -1.0,  1.0,
		-1.0, -1.0,  1.0,

		// Right face
		 1.0, -1.0, -1.0,
		 1.0,  1.0, -1.0,
		 1.0,  1.0,  1.0,
		 1.0, -1.0,  1.0,

		// Left face
		-1.0, -1.0, -1.0,
		-1.0, -1.0,  1.0,
		-1.0,  1.0,  1.0,
		-1.0,  1.0, -1.0
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
	this.position.itemSize = 3;
	this.position.numItems = 24;

	this.textureCoords = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.textureCoords);
	const textureCoords = [
	  // Front face
	  0.0, 0.0,
	  1.0, 0.0,
	  1.0, 1.0,
	  0.0, 1.0,

	  // Back face
	  1.0, 0.0,
	  1.0, 1.0,
	  0.0, 1.0,
	  0.0, 0.0,

	  // Top face
	  0.0, 1.0,
	  0.0, 0.0,
	  1.0, 0.0,
	  1.0, 1.0,

	  // Bottom face
	  1.0, 1.0,
	  0.0, 1.0,
	  0.0, 0.0,
	  1.0, 0.0,

	  // Right face
	  1.0, 0.0,
	  1.0, 1.0,
	  0.0, 1.0,
	  0.0, 0.0,

	  // Left face
	  0.0, 0.0,
	  1.0, 0.0,
	  1.0, 1.0,
	  0.0, 1.0,
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(textureCoords), gl.STATIC_DRAW);
	this.textureCoords.itemSize = 2;
	this.textureCoords.numItems = 24;

	this.index = gl.createBuffer();
	gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.index);
	const indices = [
		0, 1, 2,	  0, 2, 3,	  // Front face
		4, 5, 6,	  4, 6, 7,	  // Back face
		8, 9, 10,	  8, 10, 11,  // Top face
		12, 13, 14,   12, 14, 15, // Bottom face
		16, 17, 18,   16, 18, 19, // Right face
		20, 21, 22,   20, 22, 23  // Left face
	];
	gl.bufferData(
		gl.ELEMENT_ARRAY_BUFFER,
		new Uint16Array(indices),
		gl.STATIC_DRAW
	);
	this.index.itemSize = 1;
	this.index.numItems = 36;

	this.rotation = function(t) {
		const angle = glMatrix.toRadian(t);
		return { x: 5 * angle, y: 10 * angle, z: 2 * angle };
	};

	this.alpha = function(t) {
		return 0.4 + (0.2 * Math.sin(0.01 * t));
	};

	this.draw = function(
		perspectiveMatrix,
		t,
		sceneTranslation
	) {
		gl.useProgram(this.shaders);
		gl.disable(gl.DEPTH_TEST);
		gl.enable(gl.BLEND);
		gl.blendFunc(gl.SRC_ALPHA, gl.ONE);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, 0, sceneTranslation.z]
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [1.5, 0.0, -6.0]);
		const currentRotation = this.rotation(t);
		mat4.rotateX(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(currentRotation.x)
		);
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(currentRotation.y)
		);
		mat4.rotateZ(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(currentRotation.z)
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, this.position);
		gl.vertexAttribPointer(
			this.shaders.aVertexPosition,
			this.position.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, this.textureCoords);
		gl.vertexAttribPointer(
			this.shaders.aTextureCoord,
			this.textureCoords.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.activeTexture(this.gl.TEXTURE0);
		gl.bindTexture(gl.TEXTURE_2D, this.texture);
		gl.uniform1i(this.shaders.uSampler, 0);
		gl.uniform1f(this.shaders.uAlpha, this.alpha(t));

		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.index);

		gl.uniformMatrix4fv(
			this.shaders.perspectiveMatrix,
			false,
			perspectiveMatrix
		);
		gl.uniformMatrix4fv(
			this.shaders.modelViewMatrix,
			false,
			modelViewMatrix
		);

		gl.drawElements(gl.TRIANGLES, this.index.numItems, gl.UNSIGNED_SHORT, 0);
	};
}
