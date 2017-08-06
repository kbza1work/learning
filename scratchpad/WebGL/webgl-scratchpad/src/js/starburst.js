import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

export default class Starburst {
	constructor(gl, numSprites) {
		this.gl = gl,

		this.vao = this.gl.createVertexArray();
		this.vao.__SPECTOR_Metadata = { name: 'Starburst VAO' }
		this.gl.bindVertexArray(this.vao);

		this.initShaders();
		this.initTextures();
		this.initBuffers(numSprites);

		this.gl.bindVertexArray(null);
	}

	initShaders() {
		const shaderSrcFiles = [
			"texture_v.glsl",
			"particles_f.glsl",
		];
		const attributes = [
			"aVertexPosition",
			"aTextureCoord",
		];
		const uniforms = [
			"modelViewMatrix",
			"perspectiveMatrix",
			"uSampler",
			"uColor",
		];

		this.shaders= Util.initShaders(
			this.gl,
			shaderSrcFiles,
			attributes,
			uniforms
		);
	}

	initTextures() {
		const texture_url = "assets/textures/starburst-sprite.gif";
		this.texture = Util.initTexture(this.gl, texture_url);
	}

	initBuffers(numSprites) {
		this.position = this.gl.createBuffer();
		this.position.__SPECTOR_Metadata = { name: 'Starburst Position Buffer' }
		this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.position);
		const vertices = [
			-1.0, -1.0, 0.0,
			 1.0, -1.0, 0.0,
			-1.0,  1.0, 0.0,
			 1.0,  1.0, 0.0
		];
		this.gl.bufferData(
			this.gl.ARRAY_BUFFER,
			new Float32Array(vertices),
			this.gl.STATIC_DRAW
		);
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

		this.textureCoords = this.gl.createBuffer();
		this.textureCoords.__SPECTOR_Metadata = { name: 'Starburst Texture Buffer' }
		this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.textureCoords);
		const textureCoords = [
			0.0, 0.0,
			1.0, 0.0,
			0.0, 1.0,
			1.0, 1.0
		];
		this.gl.bufferData(
			this.gl.ARRAY_BUFFER,
			new Float32Array(textureCoords),
			this.gl.STATIC_DRAW
		);
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

		this.sprites = [];
		for(let i = 0; i < numSprites; i++) {
			const distance = i/numSprites * 1.0;
			const rotationSpeed = i/numSprites * 5.0;
			this.sprites[i] = new StarburstSprite(
				this.gl,
				distance,
				rotationSpeed,
				this.shaders,
				this.position.numItems
			);
		}
	}

	draw(perspectiveMatrix, t, sceneTranslation) {
		this.gl.useProgram(this.shaders);

		this.gl.enable(this.gl.BLEND);
		this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE);
		this.gl.disable(this.gl.DEPTH_TEST);

		this.gl.bindVertexArray(this.vao);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, sceneTranslation.y, sceneTranslation.z],
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [0.0, 0.0, -25.0]);

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(this.gl.TEXTURE_2D, this.texture);
		this.gl.uniform1i(this.shaders.uSampler, 0);

		this.gl.uniformMatrix4fv(
			this.shaders.perspectiveMatrix,
			false,
			perspectiveMatrix
		);

		this.sprites.forEach((sprite) => {
			sprite.draw(t, modelViewMatrix);
		});

		this.gl.bindVertexArray(null);
	}
}

class StarburstSprite {
	constructor(gl, distance, rotationSpeed, shaders, verticesPerStar) {
		this.gl = gl,
		this.shaders = shaders;

		this.distance = distance;
		this.rotationSpeed = rotationSpeed;

		this.verticesPerStar = verticesPerStar;

		this.color = {
			red: Math.random(),
			green: Math.random(),
			blue: Math.random()
		};
	}

	spin(t) {
		return this.rotationSpeed * glMatrix.toRadian(t);
	}

	draw(t, modelViewMatrix) {
		const spinAngle = this.spin(t);
		mat4.rotateZ(modelViewMatrix, modelViewMatrix, spinAngle);
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[10.0 * this.distance, this.distance, 0.0]
		);
		mat4.rotateZ(modelViewMatrix, modelViewMatrix, -spinAngle);

		this.gl.uniformMatrix4fv(
			this.shaders.modelViewMatrix,
			false,
			modelViewMatrix
		);
		this.gl.uniform3f(
			this.shaders.uColor,
			this.color.red,
			this.color.green,
			this.color.blue
		);

		this.gl.drawArrays(this.gl.TRIANGLE_STRIP, 0, this.verticesPerStar);
	}
}
