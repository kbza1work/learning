import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

export default class Cube {
	constructor(gl) {
		this.gl = gl;

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Cube VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initTextures();
		this.initBuffers();

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
		const texture_url = "assets/textures/glass.gif";
		this.texture = Util.initTexture(this.gl, texture_url);
	}

	initBuffers() {
		this.position = this.gl.createBuffer();
		this.position.__SPECTOR_Metadata = { name: 'Cube Position Buffer' }
		this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.position);
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
		this.gl.bufferData(
			this.gl.ARRAY_BUFFER,
			new Float32Array(vertices),
			this.gl.STATIC_DRAW
		);
		this.position.itemSize = 3;
		this.position.numItems = 24;
		this.gl.vertexAttribPointer(
			this.shaders.aVertexPosition,
			this.position.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);

		this.textureCoords = this.gl.createBuffer();
		this.textureCoords.__SPECTOR_Metadata = {
			name: 'Cube Texture Coordinate Buffer'
		}
		this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.textureCoords);
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
		this.gl.bufferData(
			this.gl.ARRAY_BUFFER,
			new Float32Array(textureCoords),
			this.gl.STATIC_DRAW
		);
		this.textureCoords.itemSize = 2;
		this.textureCoords.numItems = 24;
		this.gl.vertexAttribPointer(
			this.shaders.aTextureCoord,
			this.textureCoords.itemSize,
			this.gl.FLOAT,
			false,
			0,
			0
		);

		this.index = this.gl.createBuffer();
		this.index.__SPECTOR_Metadata = { name: 'Cube Index Buffer' }
		this.gl.bindBuffer(this.gl.ELEMENT_ARRAY_BUFFER, this.index);
		const indices = [
			0, 1, 2,	  0, 2, 3,	  // Front face
			4, 5, 6,	  4, 6, 7,	  // Back face
			8, 9, 10,	  8, 10, 11,  // Top face
			12, 13, 14,   12, 14, 15, // Bottom face
			16, 17, 18,   16, 18, 19, // Right face
			20, 21, 22,   20, 22, 23  // Left face
		];
		this.gl.bufferData(
			this.gl.ELEMENT_ARRAY_BUFFER,
			new Uint16Array(indices),
			this.gl.STATIC_DRAW
		);
		this.index.itemSize = 1;
		this.index.numItems = 36;
	}

	rotation(t) {
		const angle = glMatrix.toRadian(t);
		return { x: 5 * angle, y: 10 * angle, z: 2 * angle };
	}

	alpha(t) {
		return 0.4 + (0.2 * Math.sin(0.01 * t));
	}

	draw(perspectiveMatrix, t, sceneTranslation) {
		this.gl.useProgram(this.shaders);

		this.gl.enable(this.gl.BLEND);
		// destination alpha multiplier of ONE_MINUS_SRC_ALPHA is conventional
		// for blending, but ONE has an interesting soft glow appearance
		this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE);
		// this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE_MINUS_SRC_ALPHA);
		this.gl.disable(this.gl.DEPTH_TEST);

		this.gl.bindVertexArray(this.vao);

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

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(this.gl.TEXTURE_2D, this.texture);
		this.gl.uniform1i(this.shaders.uSampler, 0);
		this.gl.uniform1f(this.shaders.uAlpha, this.alpha(t));

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
