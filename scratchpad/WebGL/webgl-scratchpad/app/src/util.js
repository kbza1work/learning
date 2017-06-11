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

	resizeCanvas: function(canvas) {
		const fullWidth = canvas.clientWidth;
		const fullHeight = canvas.clientHeight;
		if(canvas.width !== fullWidth) {
			canvas.width = fullWidth;
		}
		if(canvas.height !== fullHeight) {
			canvas.height = fullHeight;
		}
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

			const shaderSource = require(`./shaders/${shaderFilename}`);
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

	initTexture: function(gl, textureUrl) {
		const handleLoadedTexture = function(texture) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
			// for .gif
			gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
			gl.texImage2D(
				gl.TEXTURE_2D,
				0,	// level of detail number
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

		const texture = gl.createTexture();
		texture.image = new Image();
		texture.image.onload = function() {
			handleLoadedTexture(texture);
		}
		texture.image.src = textureUrl;

		return texture;
	},
};
