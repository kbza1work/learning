import {glMatrix, mat4} from 'gl-matrix';

export default {
	// return a new WebGL context for the given canvas element, or throw an
	// exception if something went wrong
	initGL: function(canvas) {
		const gl = canvas.getContext("webgl2", { alpha: false });
		if(gl === null) {
			throw new Error("Failed to initialize WebGL context");
		}
		return gl;
	},

	// returns a boolean indicating whether a canvas resize occurred
	resizeCanvas: function(canvas) {
		let resizeOccurred = false;

		const currentWidth = canvas.clientWidth;
		const currentHeight = canvas.clientHeight;
		if(canvas.width !== currentWidth || canvas.height !== currentHeight) {
			canvas.width = currentWidth;
			canvas.height = currentHeight;

			resizeOccurred = true;
		}

		return resizeOccurred;
	},

	// calculates and returns a new perspective matrix
	perspectiveMatrix: function(aspectRatio, fovy) {
		return mat4.perspective(
			mat4.create(),
			glMatrix.toRadian(fovy),
			aspectRatio,
			0.1,
			100.0,
		);
	},

	initShaders: function(gl, shaderNames, attributeNames, uniformNames) {
		// shaderType should be a lower-case letter corresponding to the shader
		// type (e.g. "v" for vertex shader, "f" for fragment shader)
		const getShader = function(shaderFilename, shaderType) {
			let shader;
			switch(shaderType) {
				case "v":
					shader = gl.createShader(gl.VERTEX_SHADER);
					break;
				case "f":
					shader = gl.createShader(gl.FRAGMENT_SHADER);
					break;
				default:
					throw new Error(`Unrecognized shader type (${shaderType}) for shader "${shaderFilename}"`);
			}

			const shaderSource = require(`./../shaders/${shaderFilename}`);
			gl.shaderSource(shader, shaderSource);
			gl.compileShader(shader);

			if(!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
				throw new Error(`Failed to compile shader "${shaderFilename}": ${gl.getShaderInfoLog(shader)}`);
			}

			return shader;
		};

		const shaderProgram = gl.createProgram();
		shaderNames.forEach(function(filename) {
			const shaderType = filename.charAt(filename.length - ".glsl".length - 1);
			gl.attachShader(shaderProgram, getShader(filename, shaderType));
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
	},

	initTexture: function(gl, textureUrl, options = {}) {
		const handleLoadedTexture = function(texture) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
			// for .gif or jpg
			gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
			gl.texImage2D(
				gl.TEXTURE_2D,
				0,	// level of detail number
				gl.RGBA,
				gl.RGBA,
				gl.UNSIGNED_BYTE,
				texture.image
			);
			for(let optionName in options) {
				gl.texParameteri(gl.TEXTURE_2D, optionName, options[optionName]);
			}
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
			gl.texParameteri(
				gl.TEXTURE_2D,
				gl.TEXTURE_MIN_FILTER,
				gl.LINEAR_MIPMAP_NEAREST
			);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
			gl.generateMipmap(gl.TEXTURE_2D);

			gl.bindTexture(gl.TEXTURE_2D, null);
		};

		const texture = gl.createTexture();
		texture.image = new Image();
		texture.image.onload = function() {
			handleLoadedTexture(texture);
		}
		texture.image.src = textureUrl;

		return texture;
	},

	createBuffer: (gl, bufferType, DataType, data, storage_hint="STATIC_DRAW") => {
		const buffer = gl.createBuffer();
		gl.bindBuffer(gl[bufferType], buffer);
		gl.bufferData(gl[bufferType], new DataType(data), gl[storage_hint]);
		return buffer;
	},
};
