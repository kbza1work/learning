export default {
	// return a new WebGL context for the given canvas element, or throw an
	// exception if something went wrong
	initGL: function(canvas) {
		const gl = canvas.getContext("webgl2");
		gl.viewportWidth = canvas.width;
		gl.viewportHeight = canvas.height;
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

	initShaders: function(gl, shaderIDs, attributeNames, uniformNames) {
		const getShader = function(id) {
			const shaderScript = document.getElementById(id);
			if (!shaderScript) {
				return null;
			}

			let str = "";
			let k = shaderScript.firstChild;
			while (k) {
				if (k.nodeType == 3) {
					str += k.textContent;
				}
				k = k.nextSibling;
			}

			let shader;
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

		const shaderProgram = gl.createProgram();
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
