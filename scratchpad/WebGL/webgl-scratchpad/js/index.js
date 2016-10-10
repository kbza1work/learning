"use strict";

function main() {

	var FRAMES_PER_FPS_REPORT = 100;

	var COLOR_SHADERS = [
		"color-vertex-shader",
		"color-fragment-shader",
	];
	var COLOR_SHADER_ATTRIBUTES = [
		"aVertexPosition",
		"aVertexColor",
	];
	var COLOR_SHADER_UNIFORMS = [
		"modelViewMatrix",
		"perspectiveMatrix",
	];

	var TEXTURE_SHADERS = [
		"texture-vertex-shader",
		"texture-fragment-shader",
	];
	var TEXTURE_SHADER_ATTRIBUTES = [
		"aVertexPosition",
		"aTextureCoord",
	];
	var TEXTURE_SHADER_UNIFORMS = [
		"modelViewMatrix",
		"perspectiveMatrix",
		"uSampler",
	];

	// the increment in the rotation applied to the drawn shapes per frame, in
	// degrees
	var SCENE_ROTATION_STEP = 1;
	var PYRAMID_ROTATION_STEP = 2.5;
	var CUBE_ROTATION_STEP = -1;
	var CUBE_ORBIT_STEP = 0.02;

	var CUBE_TEXTURE_URL = "textures/crate.gif";

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

	function initShaders(shaderIDs, attributeNames, uniformNames) {
		var shaderProgram;

		var getShader = function(id) {
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
		};

		shaderProgram = gl.createProgram();
		shaderIDs.forEach(function(shaderID) {
			gl.attachShader(shaderProgram, getShader(shaderID));
		});
		gl.linkProgram(shaderProgram);

		if(!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
			throw new Error("Couldn't link shader program");
		}

		gl.useProgram(shaderProgram);

		attributeNames.forEach(function(shaderVarName) {
			shaderProgram[shaderVarName] =
				gl.getAttribLocation(shaderProgram, shaderVarName);
			gl.enableVertexAttribArray(shaderProgram[shaderVarName]);
		});

		uniformNames.forEach(function(shaderVarName) {
			shaderProgram[shaderVarName] =
				gl.getUniformLocation(shaderProgram, shaderVarName);
		});

		return shaderProgram;
	}

	function initPyramid(shaderProgram) {
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

	function initCube(shaderProgram, textureUrl) {
		var cubeTexture = initTexture(textureUrl);

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

		var vertexTextureCoordBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexTextureCoordBuffer);
		var textureCoords = [
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
		vertexTextureCoordBuffer.itemSize = 2;
		vertexTextureCoordBuffer.numItems = 24;

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
			texture: cubeTexture,
			textureCoords: vertexTextureCoordBuffer,
		};
	}

	function initTexture(textureUrl) {
		var handleLoadedTexture = function(texture) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
			// for .gif
			gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
			gl.texImage2D(
				gl.TEXTURE_2D,
				0,
				gl.RGBA,
				gl.RGBA,
				gl.UNSIGNED_BYTE,
				texture.image
			);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
			gl.texParameteri(
				gl.TEXTURE_2D,
				gl.TEXTURE_MIN_FILTER,
				gl.LINEAR_MIPMAP_NEAREST
			);
			gl.generateMipmap(gl.TEXTURE_2D);

			gl.bindTexture(gl.TEXTURE_2D, null);
		};

		var texture = gl.createTexture();
		texture.image = new Image();
		texture.image.onload = function() {
			handleLoadedTexture(texture);
		}
		texture.image.src = textureUrl;

		return texture;
	}

	function drawPyramid(
		shaderProgram,
		perspectiveMatrix,
		pyramid,
		sceneRotation
	) {
		gl.useProgram(shaderProgram);

		var modelViewMatrix = mat4.identity(mat4.create());
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(sceneRotation)
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [-1.5, 0.0, -5.0]);
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(pyramid.rotation)
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, pyramid.position);
		gl.vertexAttribPointer(
			shaderProgram.aVertexPositioin,
			pyramid.position.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, pyramid.color);
		gl.vertexAttribPointer(
			shaderProgram.aVertexColor,
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
		sceneRotation
	) {
		gl.useProgram(shaderProgram);

		var modelViewMatrix = mat4.identity(mat4.create());
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(sceneRotation)
		);
		mat4.translate(modelViewMatrix, modelViewMatrix, [1.5, 0.0, -5.0]);
		mat4.rotateX(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(cube.rotation[0])
		);
		mat4.rotateY(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(cube.rotation[1])
		);
		mat4.rotateZ(
			modelViewMatrix,
			modelViewMatrix,
			glMatrix.toRadian(cube.rotation[2])
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, cube.position);
		gl.vertexAttribPointer(
			shaderProgram.aVertexPosition,
			cube.position.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.bindBuffer(gl.ARRAY_BUFFER, cube.textureCoords);
		gl.vertexAttribPointer(
			shaderProgram.aTextureCoord,
			cube.textureCoords.itemSize,
			gl.FLOAT,
			false,
			0,
			0
		);

		gl.activeTexture(gl.TEXTURE0);
		gl.bindTexture(gl.TEXTURE_2D, cube.texture);
		gl.uniform1i(shaderProgram.uSampler, 0);

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

	var canvas = document.getElementById("drawing-canvas");
	resizeCanvas(canvas);
	initGL(canvas);
	var colorShaders = initShaders(
		COLOR_SHADERS,
		COLOR_SHADER_ATTRIBUTES,
		COLOR_SHADER_UNIFORMS
	);
	var textureShaders = initShaders(
		TEXTURE_SHADERS,
		TEXTURE_SHADER_ATTRIBUTES,
		TEXTURE_SHADER_UNIFORMS
	);
	var pyramid = initPyramid(colorShaders);
	var cube = initCube(textureShaders, CUBE_TEXTURE_URL);

	var sceneRotation = 0;
	var perspectiveMatrix = mat4.perspective(
		mat4.create(),
		glMatrix.toRadian(45),
		gl.viewportWidth/gl.viewportHeight,
		0.1,
		100.0
	);

	gl.clearColor(0.1, 0.2, 0.3, 1.0);
	gl.enable(gl.DEPTH_TEST);

	// the current rotation around the y-axis applied to the pyramid, in degrees
	pyramid.rotation = 0;
	// the cube's current rotation about the x-axis, y-axis, and z-axis
	// respectively, in degrees
	cube.rotation = [0, 0, 0];

	document.onkeydown = function(event) {
		var key = event.keyCode;

		switch(key) {
			case 37: // left arrow
				sceneRotation += SCENE_ROTATION_STEP;
				break;
			case 39: // right arrow
				sceneRotation -= SCENE_ROTATION_STEP;
				break;
		};
	};

	var frames_drawn_since_last_report = 0;
	var timer_start_time_ms = Date.now();
	(function drawScene() {
		window.requestAnimationFrame(drawScene);

		gl.viewport(0, 0, gl.viewportWidth, gl.viewportHeight);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

		drawPyramid(
			colorShaders,
			perspectiveMatrix,
			pyramid,
			sceneRotation
		);
		drawCube(
			textureShaders,
			perspectiveMatrix,
			cube,
			sceneRotation
		);

		pyramid.rotation = (pyramid.rotation + PYRAMID_ROTATION_STEP) % 360;
		cube.rotation = cube.rotation.map(function(degrees) {
			return (degrees + CUBE_ROTATION_STEP) % 360;
		});

		// fps counter disabled to avoid spamming the console
		// frames_drawn_since_last_report += 1;
		// if(frames_drawn_since_last_report == FRAMES_PER_FPS_REPORT) {
		// 	var now = Date.now();
		// 	var elapsed_ms = now - timer_start_time_ms;
		// 	var avg_render_time_ms = elapsed_ms/FRAMES_PER_FPS_REPORT;
		// 	var fps = 1000.0/avg_render_time_ms;
		// 	console.log("avg render time: " + avg_render_time_ms + " ms (last " + FRAMES_PER_FPS_REPORT + " frames), " + fps + " fps");
		// 	frames_drawn_since_last_report = 0;
		// 	timer_start_time_ms = now;
		// }
	})();
}

document.addEventListener('DOMContentLoaded', main);
