import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

const COLOR_SHADERS = [
	"color-vertex-shader",
	"color-fragment-shader",
];
const COLOR_SHADER_ATTRIBUTES = [
	"aVertexPosition",
	"aVertexColor",
];
const COLOR_SHADER_UNIFORMS = [
	"modelViewMatrix",
	"perspectiveMatrix",
];

export default function Pyramid(gl) {
	this.gl = gl,

	this.shaders = Util.initShaders(
		gl,
		COLOR_SHADERS,
		COLOR_SHADER_ATTRIBUTES,
		COLOR_SHADER_UNIFORMS
	);

	this.position = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.position);
	const vertices = [
		// front face
		 0.0,  1.0,  0.0,
		-1.0, -1.0,  1.0,
		 1.0, -1.0,  1.0,

		// right face
		0.0,  1.0,	0.0,
		1.0, -1.0,	1.0,
		1.0, -1.0, -1.0,

		// back face
		 0.0,  1.0,  0.0,
		 1.0, -1.0, -1.0,
		-1.0, -1.0, -1.0,

		// left face
		 0.0,  1.0,  0.0,
		-1.0, -1.0, -1.0,
		-1.0, -1.0,  1.0,
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
	this.position.itemSize = 3;
	this.position.numItems = 12;

	this.color = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.color);
	const alpha = 0.8;
	const colors = [
		// front face
		1.0, 0.0, 0.0, alpha,
		0.0, 1.0, 0.0, alpha,
		0.0, 0.0, 1.0, alpha,

		// right face
		1.0, 0.0, 0.0, alpha,
		0.0, 0.0, 1.0, alpha,
		0.0, 1.0, 0.0, alpha,

		// back face
		1.0, 0.0, 0.0, alpha,
		0.0, 1.0, 0.0, alpha,
		0.0, 0.0, 1.0, alpha,

		// left face
		1.0, 0.0, 0.0, alpha,
		0.0, 0.0, 1.0, alpha,
		0.0, 1.0, 0.0, alpha,
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);
	this.color.itemSize = 4;
	this.color.numItems = 12;

	this.rotation = function(t) {
		return glMatrix.toRadian(t);
	};

	this.draw = function(
		perspectiveMatrix,
		t,
		sceneTranslation
	) {
		gl.useProgram(this.shaders);
		gl.enable(gl.DEPTH_TEST);
		gl.disable(gl.BLEND);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, 0, sceneTranslation.z]
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [-1.5, 0.0, -5.0]);
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			this.rotation(t)
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

		gl.bindBuffer(gl.ARRAY_BUFFER, this.color);
		gl.vertexAttribPointer(
			this.shaders.aVertexColor,
			this.color.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

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

		gl.drawArrays(gl.TRIANGLES, 0, this.position.numItems);
	};
}

