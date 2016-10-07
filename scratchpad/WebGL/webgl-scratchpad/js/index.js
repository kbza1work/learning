// return a new WebGL context for the given canvas element, or throw an
// exception if something went wrong
function initGL(canvas) {
	var gl;
	gl = canvas.getContext("experimental-webgl");
	gl.viewportWidth = canvas.width;
	gl.viewportHeight = canvas.height;
	if(gl === null) {
		throw new Error("Failed to get WebGL context");
	}
	return gl;
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

function getShader(gl, id) {
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


function initShaders(gl) {
	var shaderProgram;

	var fragmentShader = getShader(gl, "vertex-shader");
	var vertexShader = getShader(gl, "fragment-shader");

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

function initTriangleBuffers(gl) {
	var triangleVertexPositionBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, triangleVertexPositionBuffer);
	var vertices = [
		 0.0,  1.0,  0.0,
		-1.0, -1.0,  0.0,
		 1.0, -1.0,  0.0
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
	triangleVertexPositionBuffer.itemSize = 3;
	triangleVertexPositionBuffer.numItems = 3;

	var triangleVertexColorBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, triangleVertexColorBuffer);
	var colors = [
		1.0, 0.0, 0.0, 1.0,
		0.0, 1.0, 0.0, 1.0,
		0.0, 0.0, 1.0, 1.0,
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);
	triangleVertexColorBuffer.itemSize = 4;
	triangleVertexColorBuffer.numItems = 3;

	return {
		position: triangleVertexPositionBuffer,
		color: triangleVertexColorBuffer
	};
}

function initSquareBuffers(gl) {
	var squareVertexPositionBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, squareVertexPositionBuffer);
	vertices = [
		 1.0,  1.0,  0.0,
		-1.0,  1.0,  0.0,
		 1.0, -1.0,  0.0,
		-1.0, -1.0,  0.0
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
	squareVertexPositionBuffer.itemSize = 3;
	squareVertexPositionBuffer.numItems = 4;

	var squareVertexColorBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, squareVertexColorBuffer);
	var colors = [
		0.5, 0.5, 1.0, 1.0,
		0.5, 0.5, 1.0, 1.0,
		0.5, 0.5, 1.0, 1.0,
		0.5, 0.5, 1.0, 1.0,
	];
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);
	squareVertexColorBuffer.itemSize = 4;
	squareVertexColorBuffer.numItems = 4;

	return {
		position: squareVertexPositionBuffer,
		color: squareVertexColorBuffer
	};
}

function drawTriangle(
	gl,
	shaderProgram,
	perspectiveMatrix,
	triangleBuffers,
	rotation
) {
	var modelViewMatrix = mat4.identity(mat4.create());
	mat4.translate(modelViewMatrix, modelViewMatrix, [-1.5, 0.0, -5.0]);
	mat4.rotateY(modelViewMatrix, modelViewMatrix, glMatrix.toRadian(rotation));

	gl.bindBuffer(gl.ARRAY_BUFFER, triangleBuffers.position);
	gl.vertexAttribPointer(
		shaderProgram.vertexPositionAttribute,
		triangleBuffers.position.itemSize,
		gl.FLOAT,
		false,
		0,
		0
	);

	gl.bindBuffer(gl.ARRAY_BUFFER, triangleBuffers.color);
	gl.vertexAttribPointer(
		shaderProgram.vertexColorAttribute,
		triangleBuffers.color.itemSize,
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

	gl.drawArrays(gl.TRIANGLES, 0, triangleBuffers.position.numItems);
}

function drawSquare(
	gl,
	shaderProgram,
	perspectiveMatrix,
	squareBuffers,
	rotation
) {
	var modelViewMatrix = mat4.identity(mat4.create());
	mat4.translate(modelViewMatrix, modelViewMatrix, [1.5, 0.0, -5.0]);
	mat4.rotateX(modelViewMatrix, modelViewMatrix, glMatrix.toRadian(rotation));

	gl.bindBuffer(gl.ARRAY_BUFFER, squareBuffers.position);
	gl.vertexAttribPointer(
		shaderProgram.vertexPositionAttribute,
		squareBuffers.position.itemSize,
		gl.FLOAT,
		false,
		0,
		0
	);

	gl.bindBuffer(gl.ARRAY_BUFFER, squareBuffers.color);
	gl.vertexAttribPointer(
		shaderProgram.vertexColorAttribute,
		squareBuffers.color.itemSize,
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
	gl.drawArrays(gl.TRIANGLE_STRIP, 0, squareBuffers.position.numItems);
}


function main() {
	// the increment in the rotation applied to the drawn shapes per frame, in
	// degrees
	var triangle_rotation_step = 2.0;
	var square_rotation_step = 2.0;

	var canvas = document.getElementById("drawing-canvas");
	resizeCanvas(canvas);
	var gl = initGL(canvas);
	var shaderProgram = initShaders(gl);
	var triangleBuffers = initTriangleBuffers(gl);
	var squareBuffers = initSquareBuffers(gl);
	gl.clearColor(0.1, 0.2, 0.3, 1.0);
	gl.enable(gl.DEPTH_TEST);

	// the current rotation applied to each drawn shape, in degrees
	var rotationTriangle = 0;
	var rotationSquare = 0;

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

		drawTriangle(
			gl,
			shaderProgram,
			perspectiveMatrix,
			triangleBuffers,
			rotationTriangle
		);
		drawSquare(
			gl,
			shaderProgram,
			perspectiveMatrix,
			squareBuffers,
			rotationSquare
		);

		rotationTriangle += triangle_rotation_step;
		rotationSquare += square_rotation_step;
	})();
}

document.addEventListener('DOMContentLoaded', main);
