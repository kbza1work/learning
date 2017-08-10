"use strict";

import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

export default class Flame {
	constructor(gl, position, color, numParticles) {
		this.gl = gl;

		this.color = color;
		this.position = position;
		this.numParticles = numParticles;

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Flame VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initTextures();
		this.initBuffers();

		this.gl.bindVertexArray(null);
	}

	initShaders() {
		const shaderSrcFiles = [
			"flame_v.glsl",
			"flame_f.glsl",
		];
		const attributes = [
			"aLifetime",
			"aTextureCoords",
			"aTriCorner",
			"aCenterOffset",
			"aVelocity",
		];
		const uniforms = [
			"modelViewMatrix",
			"perspectiveMatrix",
			"uTime",
			"uFirePos",
			"uColor",
			"uTimeFrag",
			"fireAtlas",
		];

		this.shaders = Util.initShaders(
			this.gl,
			shaderSrcFiles,
			attributes,
			uniforms
		);
	}

	initTextures() {
		const texture_url = "assets/textures/fire-texture-atlas.jpg";
		this.texture = Util.initTexture(
			this.gl,
			texture_url,
			{ [this.gl.TEXTURE_MIN_FILTER]: this.gl.LINEAR },
		);
	}

	initBuffers() {
		let lifetimes = []
		let triCorners = []
		let texCoords = []
		let vertexIndices = []
		let centerOffsets = []
		let velocities = []

		const triCornersCycle = [
			-1.0, -1.0,
			 1.0, -1.0,
			 1.0, 1.0,
			-1.0, 1.0
		]
		const texCoordsCycle = [
			0, 0,
			1, 0,
			1, 1,
			0, 1
		]

		for(let i = 0; i < this.numParticles; i++) {
			const lifetime = 8 * Math.random()

			const diameterAroundCenter = 0.5
			const halfDiameterAroundCenter = diameterAroundCenter / 2

			let xStartOffset = diameterAroundCenter * Math.random() - halfDiameterAroundCenter;
			xStartOffset /= 3

			let yStartOffset = diameterAroundCenter * Math.random() - halfDiameterAroundCenter;
			yStartOffset /= 10;

			let zStartOffset = diameterAroundCenter * Math.random() - halfDiameterAroundCenter;
			zStartOffset /= 3;

			const upVelocity = 0.1 * Math.random()

			let xSideVelocity = 0.02 * Math.random()
			if (xStartOffset > 0) {
				xSideVelocity *= -1
			}

			let zSideVelocity = 0.02 * Math.random()
			if (zStartOffset > 0) {
				zSideVelocity *= -1
			}

			for(let j = 0; j < 4; j++) {
				lifetimes.push(lifetime)

				triCorners.push(triCornersCycle[j * 2])
				triCorners.push(triCornersCycle[j * 2 + 1])

				texCoords.push(texCoordsCycle[j * 2])
				texCoords.push(texCoordsCycle[j * 2 + 1])
				
				centerOffsets.push(xStartOffset)
				centerOffsets.push(yStartOffset + Math.abs(xStartOffset / 2.0))
				centerOffsets.push(zStartOffset)

				velocities.push(xSideVelocity)
				velocities.push(upVelocity)
				velocities.push(zSideVelocity)
			}

			vertexIndices = vertexIndices.concat([
				0, 1, 2, 0, 2, 3
			].map(function (num) { return num + 4 * i }))
		}

		Util.createBuffer(this.gl, 'ARRAY_BUFFER', Float32Array, lifetimes);
		this.gl.vertexAttribPointer(this.shaders.aLifetime, 1, this.gl.FLOAT, false, 0, 0);

		Util.createBuffer(this.gl, 'ARRAY_BUFFER', Float32Array, texCoords);
		this.gl.vertexAttribPointer(this.shaders.aTextureCoords, 2, this.gl.FLOAT, false, 0, 0);

		Util.createBuffer(this.gl, 'ARRAY_BUFFER', Float32Array, triCorners);
		this.gl.vertexAttribPointer(this.shaders.aTriCorner, 2, this.gl.FLOAT, false, 0, 0);

		Util.createBuffer(this.gl, 'ARRAY_BUFFER', Float32Array, centerOffsets);
		this.gl.vertexAttribPointer(this.shaders.aCenterOffset, 3, this.gl.FLOAT, false, 0, 0);

		Util.createBuffer(this.gl, 'ARRAY_BUFFER', Float32Array, velocities);
		this.gl.vertexAttribPointer(this.shaders.aVelocity, 3, this.gl.FLOAT, false, 0, 0);

		Util.createBuffer(this.gl, 'ELEMENT_ARRAY_BUFFER', Uint16Array, vertexIndices);

		this.gl.uniform1i(this.shaders.fireAtlas, 0);
	}

	draw(perspectiveMatrix, t, sceneTranslation) {
		this.gl.useProgram(this.shaders);

		this.gl.enable(this.gl.BLEND);
		this.gl.blendFunc(this.gl.ONE, this.gl.ONE);
		this.gl.disable(this.gl.DEPTH_TEST);

		this.gl.bindVertexArray(this.vao);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, sceneTranslation.y, sceneTranslation.z],
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, this.position);

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(this.gl.TEXTURE_2D, this.texture);

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

		this.gl.uniform1f(this.shaders.uTime, t/8.0);
		this.gl.uniform1f(this.shaders.uTimeFrag, t);

		this.gl.uniform3fv(this.shaders.uFirePos, this.position);
		this.gl.uniform4fv(this.shaders.uColor, this.color);

		this.gl.drawElements(
			this.gl.TRIANGLES,
			this.numParticles * 6,
			this.gl.UNSIGNED_SHORT,
			0,
		);

		this.gl.bindVertexArray(null);
	}
}
