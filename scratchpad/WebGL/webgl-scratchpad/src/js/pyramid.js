import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

export default class Pyramid {
	constructor(gl) {
		this.gl = gl,

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Pyramid VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initBuffers();

		this.gl.bindVertexArray(null);
	}

	initShaders() {
		const shaderSrcFiles = [
			"color_v.glsl",
			"color_f.glsl",
		];
		const attributes = [
			"aVertexPosition",
			"aVertexColor",
		];
		const uniforms = [
			"modelViewMatrix",
			"perspectiveMatrix",
		];

		this.shaders = Util.initShaders(
			this.gl,
			shaderSrcFiles,
			attributes,
			uniforms
		);
	};

	initBuffers() {
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
		this.position =
			Util.createBuffer(this.gl, "ARRAY_BUFFER", Float32Array, vertices);
		this.position.__SPECTOR_Metadata = { name: 'Pyramid Position Buffer' }
		this.position.itemSize = 3;
		this.position.numItems = 12;
		this.gl.vertexAttribPointer(
			this.shaders.aVertexPosition,
			this.position.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);

		const alpha = 1.0;	// note that alpha will have no effect as long as blending is off
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
		this.color =
			Util.createBuffer(this.gl, "ARRAY_BUFFER", Float32Array, colors);
		this.color.__SPECTOR_Metadata = { name: 'Cube Color Buffer' }
		this.color.itemSize = 4;
		this.color.numItems = 12;
		this.gl.vertexAttribPointer(
			this.shaders.aVertexColor,
			this.color.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);
	}

	rotation(t) {
		return glMatrix.toRadian(t);
	}

	draw(perspectiveMatrix, t, sceneTranslation) {
		this.gl.useProgram(this.shaders);

		this.gl.disable(this.gl.BLEND);
		this.gl.enable(this.gl.DEPTH_TEST);

		this.gl.bindVertexArray(this.vao);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, sceneTranslation.y, sceneTranslation.z],
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [-1.5, 0.0, -5.0]);
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			this.rotation(t)
		);

		this.gl.uniformMatrix4fv(
			this.shaders.perspectiveMatrix,
			false,
			perspectiveMatrix
		);
		this.gl.uniformMatrix4fv(
			this.shaders.modelViewMatrix,
			false,
			modelViewMatrix
		);

		this.gl.drawArrays(this.gl.TRIANGLES, 0, this.position.numItems);

		this.gl.bindVertexArray(null);
	}
}
