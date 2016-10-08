"use strict";

function main() {
	var gl;

	// return a new WebGL context for the given canvas element, or throw an
	// exception if something went wrong
	function initGL(canvas) {
		gl = canvas.getContext("experimental-webgl");
		gl.viewportWidth = canvas.width;
		gl.viewportHeight = canvas.height;
		if(gl === null) {
			throw new Error("Failed to get WebGL context");
		}
	}

	function resizeCanvas(canvas) {
		var fullWidth = canvas.clientWidth;
		var fullHeight = canvas.clientHeight;
		if(canvas.width !== fullWidth) {
			canvas.width = fullWidth;
		}
		if(canvas.height !== fullHeight) {
			canvas.height = fullHeight;
		}
	}

	function getShader(id) {
		var shaderScript = document.getElementById(id);
		if (!shaderScript) {
			return null;
		}

		var str = "";
		var k = shaderScript.firstChild;
		while (k) {
			if (k.nodeType == 3) {
				str += k.textContent;
			}
			k = k.nextSibling;
		}

		var shader;
		if (shaderScript.type == "x-shader/x-fragment") {
			shader = gl.createShader(gl.FRAGMENT_SHADER);
		} else if (shaderScript.type == "x-shader/x-vertex") {
			shader = gl.createShader(gl.VERTEX_SHADER);
		} else {
			throw new Error("Unrecognized shader MIME type: " + shaderScript.type);
		}

		gl.shaderSource(shader, str);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			throw new Error(gl.getShaderInfoLog(shader));
		}

		return shader;
	}


	function initShaders() {
		var shaderProgram;

		var fragmentShader = getShader("vertex-shader");
		var vertexShader = getShader("fragment-shader");

		shaderProgram = gl.createProgram();
		gl.attachShader(shaderProgram, vertexShader);
		gl.attachShader(shaderProgram, fragmentShader);
		gl.linkProgram(shaderProgram);

		if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
			throw new Error("Couldn't link shader program");
		}

		gl.useProgram(shaderProgram);

		shaderProgram.vertexPositionAttribute =
			gl.getAttribLocation(shaderProgram, "aVertexPosition");
		gl.enableVertexAttribArray(shaderProgram.vertexPositionAttribute);
		shaderProgram.vertexColorAttribute =
			gl.getAttribLocation(shaderProgram, "aVertexColor");
		gl.enableVertexAttribArray(shaderProgram.vertexColorAttribute);

		shaderProgram.perspectiveMatrix =
			gl.getUniformLocation(shaderProgram, "perspectiveMatrix");
		shaderProgram.modelViewMatrix =
			gl.getUniformLocation(shaderProgram, "modelViewMatrix");

		return shaderProgram;
	}

	function initPyramid() {
		var vertexPositionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexPositionBuffer);
		var vertices = [
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
		vertexPositionBuffer.itemSize = 3;
		vertexPositionBuffer.numItems = 12;

		var vertexColorBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexColorBuffer);
		var colors = [
			// front face
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,

			// right face
			1.0, 0.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 0.0, 1.0,

			// back face
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,

			// left face
			1.0, 0.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
		];
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);
		vertexColorBuffer.itemSize = 4;
		vertexColorBuffer.numItems = 12;

		return {
			position: vertexPositionBuffer,
			color: vertexColorBuffer
		};
	}

	function initCube() {
		var vertexPositionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexPositionBuffer);
		var vertices = [
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
		vertexPositionBuffer.itemSize = 3;
		vertexPositionBuffer.numItems = 24;

		var vertexColorBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexColorBuffer);
		var colors = [
			[1.0, 0.0, 0.0, 1.0], // Front face
			[1.0, 1.0, 0.0, 1.0], // Back face
			[0.0, 1.0, 0.0, 1.0], // Top face
			[1.0, 0.5, 0.5, 1.0], // Bottom face
			[1.0, 0.0, 1.0, 1.0], // Right face
			[0.0, 0.0, 1.0, 1.0]  // Left face
		];
		var unpackedColors = [];
		for (var i in colors) {
			var color = colors[i];
			for (var j=0; j < 4; j++) {
				unpackedColors = unpackedColors.concat(color);
			}
		}
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(unpackedColors), gl.STATIC_DRAW);
		vertexColorBuffer.itemSize = 4;
		vertexColorBuffer.numItems = 24;

		var indexBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
		var indices = [
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
		indexBuffer.itemSize = 1;
		indexBuffer.numItems = 36;

		return {
			position: vertexPositionBuffer,
			index: indexBuffer,
			color: vertexColorBuffer,
		};
	}

	function drawPyramid(
		shaderProgram,
		perspectiveMatrix,
		pyramid,
		rotation
	) {
		var modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(modelViewMatrix, modelViewMatrix, [-1.5, 0.0, -5.0]);
		mat4.rotateY(modelViewMatrix, modelViewMatrix, glMatrix.toRadian(rotation));

		gl.bindBuffer(gl.ARRAY_BUFFER, pyramid.position);
		gl.vertexAttribPointer(
			shaderProgram.vertexPositionAttribute,
			pyramid.position.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, pyramid.color);
		gl.vertexAttribPointer(
			shaderProgram.vertexColorAttribute,
			pyramid.color.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.uniformMatrix4fv(
			shaderProgram.perspectiveMatrix,
			false,
			perspectiveMatrix
		);
		gl.uniformMatrix4fv(
			shaderProgram.modelViewMatrix,
			false,
			modelViewMatrix
		);

		gl.drawArrays(gl.TRIANGLES, 0, pyramid.position.numItems);
	}

	function drawCube(
		shaderProgram,
		perspectiveMatrix,
		cube,
		rotation
	) {
		var modelViewMatrix = mat4.identity(mat4.create());
		mat4.translate(modelViewMatrix, modelViewMatrix, [1.5, 0.0, -5.0]);
		mat4.rotate(modelViewMatrix, modelViewMatrix, glMatrix.toRadian(rotation), [1, 1, 1]);

		gl.bindBuffer(gl.ARRAY_BUFFER, cube.position);
		gl.vertexAttribPointer(
			shaderProgram.vertexPositionAttribute,
			cube.position.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, cube.color);
		gl.vertexAttribPointer(
			shaderProgram.vertexColorAttribute,
			cube.color.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, cube.index);

		gl.uniformMatrix4fv(
			shaderProgram.perspectiveMatrix,
			false,
			perspectiveMatrix
		);
		gl.uniformMatrix4fv(
			shaderProgram.modelViewMatrix,
			false,
			modelViewMatrix
		);
		gl.drawElements(gl.TRIANGLES, cube.index.numItems, gl.UNSIGNED_SHORT, 0);
	}

	// the increment in the rotation applied to the drawn shapes per frame, in
	// degrees
	var PYRAMID_ROTATION_STEP = 2.5;
	var CUBE_ROTATION_STEP = -1;

	var canvas = document.getElementById("drawing-canvas");
	resizeCanvas(canvas);
	initGL(canvas);
	var shaderProgram = initShaders();
	var pyramid = initPyramid();
	var cube = initCube();

	gl.clearColor(0.1, 0.2, 0.3, 1.0);
	gl.enable(gl.DEPTH_TEST);

	// the current rotation applied to each drawn shape, in degrees
	var rotationPyramid = 0;
	var rotationCube = 90;

	(function drawScene() {
		window.requestAnimationFrame(drawScene);

		gl.viewport(0, 0, gl.viewportWidth, gl.viewportHeight);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

		var perspectiveMatrix = mat4.perspective(
			mat4.create(),
			glMatrix.toRadian(90),
			gl.viewportWidth/gl.viewportHeight,
			0.1,
			100.0
		);

		drawPyramid(
			shaderProgram,
			perspectiveMatrix,
			pyramid,
			rotationPyramid
		);
		drawCube(
			shaderProgram,
			perspectiveMatrix,
			cube,
			rotationCube
		);

		rotationPyramid = (rotationPyramid + PYRAMID_ROTATION_STEP) % 360;
		rotationCube = (rotationCube + CUBE_ROTATION_STEP) % 360;
	})();
}

document.addEventListener('DOMContentLoaded', main);
