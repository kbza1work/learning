"use strict";

import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

export default class Ground {
	constructor(gl, zPosition) {
		this.gl = gl;

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Ground VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initTextures();
		this.initBuffers(zPosition);

		this.gl.bindVertexArray(null);
	}

	initShaders() {
		const shaderSrcFiles = [
			"texture_v.glsl",
			"texture_f.glsl",
		];
		const attributes = [
			"aVertexPosition",
			"aTextureCoord",
		];
		const uniforms = [
			"modelViewMatrix",
			"perspectiveMatrix",
			"uSampler",
			"uAlpha",
		];

		this.shaders = Util.initShaders(
			this.gl,
			shaderSrcFiles,
			attributes,
			uniforms
		);
	}

	initTextures() {
		const texture_url = "assets/textures/grass.jpg";
		this.texture = Util.initTexture(
			this.gl,
			texture_url,
			{ [this.gl.TEXTURE_MIN_FILTER]: this.gl.LINEAR_MIPMAP_NEAREST },
		);
	}

	initBuffers(zPosition) {
		const vertices = [
			-100.0, zPosition,  100.0,
			 100.0, zPosition,  100.0,
			 100.0, zPosition, -100.0,
			-100.0, zPosition, -100.0,
		];
		this.position =
			Util.createBuffer(this.gl, "ARRAY_BUFFER", Float32Array, vertices);
		this.position.__SPECTOR_Metadata = { name: 'Ground Position Buffer' };
		this.position.itemSize = 3;
		this.position.numItems = 4;
		this.gl.vertexAttribPointer(
			this.shaders.aVertexPosition,
			this.position.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);

		const textureCoords = [
		  0.0, 0.0,
		  100.0, 0.0,
		  100.0, 100.0,
		  0.0, 100.0,
		];
		this.textureCoords =
			Util.createBuffer(this.gl, "ARRAY_BUFFER", Float32Array, textureCoords);
		this.textureCoords.__SPECTOR_Metadata = {
			name: 'Ground Texture Coordinate Buffer'
		};
		this.textureCoords.itemSize = 2;
		this.textureCoords.numItems = 4;
		this.gl.vertexAttribPointer(
			this.shaders.aTextureCoord,
			this.textureCoords.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);

		const indices = [
			0, 1, 2,
			0, 2, 3,
		];
		this.index =
			Util.createBuffer(this.gl, "ELEMENT_ARRAY_BUFFER", Uint16Array, indices);
		this.index.__SPECTOR_Metadata = { name: 'Ground Index Buffer' };
		this.index.itemSize = 1;
		this.index.numItems = 6;
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

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(this.gl.TEXTURE_2D, this.texture);
		this.gl.uniform1i(this.shaders.uSampler, 0);
		this.gl.uniform1f(this.shaders.uAlpha, 1.0);

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

		this.gl.drawElements(
			this.gl.TRIANGLES,
			this.index.numItems,
			this.gl.UNSIGNED_SHORT,
			0
		);

		this.gl.bindVertexArray(null);
	}
}
