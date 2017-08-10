"use strict";

import {glMatrix, mat4, vec2} from 'gl-matrix';

import Util from './util';

export default class FixedText {
	constructor(gl) {
		this.gl = gl;

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Fixed Text VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initTextures();
		this.initBuffers();

		this.gl.bindVertexArray(null);
	}

	initShaders() {
		const shaderSrcFiles = [
			"signed_distance_font_v.glsl",
			"signed_distance_font_f.glsl",
		];
		const attributes = [
			"aVertexPosition",
			"aTextureCoordinate",
		];
		const uniforms = [
			"modelViewMatrix",
			"projectionMatrix",
			"msdf",
			"color",
			"bgColor",
			"pxRange",
		];

		this.shaders = Util.initShaders(
			this.gl,
			shaderSrcFiles,
			attributes,
			uniforms
		);
	}

	initTextures() {
		const texture_url = "assets/textures/ubuntu-mono-sdf-atlas-32.png";
		this.texture = Util.initTexture(
			this.gl,
			texture_url,
			{
				[this.gl.TEXTURE_MIN_FILTER]: this.gl.LINEAR,
				[this.gl.TEXTURE_MAG_FILTER]: this.gl.LINEAR,
				// DEBUG
				// [this.gl.TEXTURE_WRAP_S]: this.gl.CLAMP_TO_EDGE,
				// [this.gl.TEXTURE_WRAP_T]: this.gl.CLAMP_TO_EDGE,
			},
		);
		this.texture.baseSdfSize = 32;
		this.texture.totalGlyphWidth = (this.texture.baseSdfSize + 2);
		this.texture.glyphsPerRow = 64;
		this.texture.totalGlyphs = 1280;
		this.texture.fontAtlasDimensions = vec2.fromValues(
			this.texture.totalGlyphWidth * this.texture.glyphsPerRow,
			this.texture.totalGlyphWidth * Math.ceil(this.texture.totalGlyphs / this.texture.glyphsPerRow),
		);
	}

	initBuffers() {
		const vertices = [
			0.0, 0.0, -0.1,
			1.0, 0.0, -0.1,
			1.0, 1.0, -0.1,
			0.0, 1.0, -0.1,
		];
		this.position =
			Util.createBuffer(this.gl, "ARRAY_BUFFER", Float32Array, vertices);
		this.position.__SPECTOR_Metadata = { name: 'Fixed Text Position Buffer' };
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

		const indices = [
			0, 1, 2,
			0, 2, 3,
		];
		this.index =
			Util.createBuffer(this.gl, "ELEMENT_ARRAY_BUFFER", Uint16Array, indices);
		this.index.__SPECTOR_Metadata = { name: 'Fixed Text Index Buffer' };
		this.index.itemSize = 1;
		this.index.numItems = 6;
	}

	textureCoordinates(codePoint) {
		const bottomLeftCorner = vec2.fromValues(
			((codePoint - 1) % this.texture.glyphsPerRow) * this.texture.totalGlyphWidth,
			this.texture.fontAtlasDimensions[1] - (Math.ceil(codePoint / this.texture.glyphsPerRow) * this.texture.totalGlyphWidth)
		);

		// DEBUG
		// return [
		// 	bottomLeftCorner[0], bottomLeftCorner[1],	// bottom left corner
		// 	bottomLeftCorner[0] + this.texture.totalGlyphWidth, bottomLeftCorner[1], // bottom right corner
		// 	bottomLeftCorner[0] + this.texture.totalGlyphWidth, bottomLeftCorner[1] + this.texture.totalGlyphWidth, // top right corner
		// 	bottomLeftCorner[0], bottomLeftCorner[1] + this.texture.totalGlyphWidth, // top left corner
		// ];
		return [
			bottomLeftCorner[0] / this.texture.fontAtlasDimensions[0], bottomLeftCorner[1] / this.texture.fontAtlasDimensions[1],	// bottom left corner
			(bottomLeftCorner[0] + this.texture.totalGlyphWidth) / this.texture.fontAtlasDimensions[0], bottomLeftCorner[1] / this.texture.fontAtlasDimensions[1], // bottom right corner
			(bottomLeftCorner[0] + this.texture.totalGlyphWidth) / this.texture.fontAtlasDimensions[0], (bottomLeftCorner[1] + this.texture.totalGlyphWidth) / this.texture.fontAtlasDimensions[1], // top right corner
			bottomLeftCorner[0] / this.texture.fontAtlasDimensions[0], (bottomLeftCorner[1] + this.texture.totalGlyphWidth) / this.texture.fontAtlasDimensions[1], // top left corner
		];
	}

	draw(position, text, fontSize, color, bgColor) {
		this.gl.useProgram(this.shaders);

		this.gl.disable(this.gl.BLEND);
		this.gl.enable(this.gl.DEPTH_TEST);

		this.gl.bindVertexArray(this.vao);

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(this.gl.TEXTURE_2D, this.texture);
		this.gl.uniform1i(this.shaders.msdf, 0);

		// this is set when generating the multi-channel signed distance
		// field--the msdf generator script in this repo uses a value of 1 by
		// default
		this.gl.uniform1f(this.shaders.pxRange, 1.0);

		const screenWidth = this.gl.drawingBufferWidth;
		const screenHeight = this.gl.drawingBufferHeight;
		let projectionMatrix = mat4.create();
		mat4.ortho(projectionMatrix, 0.0, screenWidth, 0.0, screenHeight, 0.1, 1000);
		this.gl.uniformMatrix4fv(
			this.shaders.projectionMatrix,
			false,
			projectionMatrix
		);

		this.gl.uniform4fv(
			this.shaders.color,
			color,
		);
		this.gl.uniform4fv(
			this.shaders.bgColor,
			bgColor,
		);

		for(var i = 0; i < text.length; i++) {
			// note that this only works for characters in the basic
			// multilingual plane; to use all Unicode characters use
			// codePointAt() and don't iterate the string assuming each
			// character is a single unit in length
			const codePoint = text.charCodeAt(i);

			const textureCoords = this.textureCoordinates(codePoint);
			const textureCoordBuffer = Util.createBuffer(
				this.gl,
				"ARRAY_BUFFER",
				Float32Array,
				textureCoords,
				"DYNAMIC_DRAW"
			);
			textureCoordBuffer.__SPECTOR_Metadata =
				{ name: 'Fixed Text Texture Coordinates Buffer' };
			textureCoordBuffer.itemSize = 2;
			textureCoordBuffer.numItems = 4;
			this.gl.vertexAttribPointer(
				this.shaders.aTextureCoordinate,
				textureCoordBuffer.itemSize,
				this.gl.FLOAT,
				false,
				0,
				0
			);

			let modelViewMatrix = mat4.identity(mat4.create());
			mat4.translate(
				modelViewMatrix,
				modelViewMatrix,
				[position[0], position[1], 0.0],
			);
			mat4.translate(
				modelViewMatrix,
				modelViewMatrix,
				[fontSize * i, 0.0, 0.0],
			);
			mat4.scale(
				modelViewMatrix,
				modelViewMatrix,
				[fontSize, fontSize, 1.0],
			);
			this.gl.uniformMatrix4fv(
				this.shaders.modelViewMatrix,
				false,
				modelViewMatrix,
			);

			this.gl.drawElements(
				this.gl.TRIANGLES,
				this.index.numItems,
				this.gl.UNSIGNED_SHORT,
				0
			);
		}

		this.gl.bindVertexArray(null);
	}
}
