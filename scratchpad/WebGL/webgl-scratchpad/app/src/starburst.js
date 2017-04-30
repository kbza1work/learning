import {glMatrix, mat4} from 'gl-matrix';

import Util from './util';

const STAR_TEXTURE_URL = "assets/textures/starburst-sprite.gif";

const STARBURST_SHADERS = [
	"texture-vertex-shader",
	"starburst-fragment-shader",
];

const STARBURST_SHADER_ATTRIBUTES = [
	"aVertexPosition",
	"aTextureCoord",
];
const STARBURST_SHADER_UNIFORMS = [
	"modelViewMatrix",
	"perspectiveMatrix",
	"uSampler",
	"uColor",
];

export default function Starburst(gl, numSprites) {
	this.gl = gl,

	this.vao = gl.createVertexArray();
	gl.bindVertexArray(this.vao);

	this.shaders = Util.initShaders(
		gl,
		STARBURST_SHADERS,
		STARBURST_SHADER_ATTRIBUTES,
		STARBURST_SHADER_UNIFORMS
	);

	this.texture = Util.initTexture(this.gl, STAR_TEXTURE_URL);

	this.position = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.position);
	const vertices = [
		-1.0, -1.0, 0.0,
		 1.0, -1.0, 0.0,
		-1.0,  1.0, 0.0,
		 1.0,  1.0, 0.0
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
	this.position.itemSize = 3;
	this.position.numItems = 4;
	this.gl.vertexAttribPointer(
		this.shaders.aVertexPosition,
		this.position.itemSize,
		gl.FLOAT,
		false,
		0,
		0
	);

	this.textureCoords = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, this.textureCoords);
	const textureCoords = [
		0.0, 0.0,
		1.0, 0.0,
		0.0, 1.0,
		1.0, 1.0
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(textureCoords), gl.STATIC_DRAW);
	this.textureCoords.itemSize = 2;
	this.textureCoords.numItems = 4;
	this.gl.vertexAttribPointer(
		this.shaders.aTextureCoord,
		this.textureCoords.itemSize,
		gl.FLOAT,
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

	gl.bindVertexArray(null);

	this.draw = function(
		perspectiveMatrix,
		t,
		sceneTranslation
	) {
		gl.useProgram(this.shaders);
		gl.disable(gl.DEPTH_TEST);
		gl.enable(gl.BLEND);
		gl.blendFunc(gl.SRC_ALPHA, gl.ONE);
		gl.bindVertexArray(this.vao);

		let modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(
			modelViewMatrix,
			modelViewMatrix,
			[sceneTranslation.x, 0, sceneTranslation.z]
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [0.0, 0.0, -25.0]);

		this.gl.activeTexture(this.gl.TEXTURE0);
		this.gl.bindTexture(gl.TEXTURE_2D, this.texture);
		this.gl.uniform1i(this.shaders.uSampler, 0);

		gl.uniformMatrix4fv(
			this.shaders.perspectiveMatrix,
			false,
			perspectiveMatrix
		);

		this.sprites.forEach((sprite) => {
			sprite.draw(t, modelViewMatrix);
		});

		gl.bindVertexArray(null);
	};
}

function StarburstSprite(gl, distance, rotationSpeed, shaders, verticesPerStar) {

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

	this.spin = function(t) {
		return this.rotationSpeed * glMatrix.toRadian(t);
	};

	this.draw = function(t, modelViewMatrix) {
		const spinAngle = this.spin(t);
		mat4.rotateZ(modelViewMatrix, modelViewMatrix, spinAngle);
		mat4.translate(modelViewMatrix, modelViewMatrix, [10.0 * this.distance, this.distance, 0.0]);
		mat4.rotateZ(modelViewMatrix, modelViewMatrix, -spinAngle);

		gl.uniformMatrix4fv(
			this.shaders.modelViewMatrix,
			false,
			modelViewMatrix
		);
		gl.uniform3f(
			this.shaders.uColor,
			this.color.red,
			this.color.green,
			this.color.blue
		);

		this.gl.drawArrays(gl.TRIANGLE_STRIP, 0, this.verticesPerStar);
	};
}

