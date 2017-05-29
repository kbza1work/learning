"use strict";

import {glMatrix, mat4} from 'gl-matrix';

import Settings from './constants';
import Util from './util';
import Pyramid from './pyramid';
import Cube from './cube';
import Starburst from './starburst';

function main() {
	const canvas = document.getElementById(Settings.WEBGL_CANVAS_ID);
	if(canvas === null) {
		console.error(
			`Couldn't find drawing canvas for WebGL scene, check that a ` +
			`canvas element with id "${Settings.WEBGL_CANVAS_ID}" exists.`
		);
		return;
	}
	Util.resizeCanvas(canvas);
	const gl = Util.initGL(canvas);
	console.log(`Acquired WebGL context.`);

	const pyramid = Settings.ENABLE_PYRAMID ? new Pyramid(gl) : null;
	const cube = Settings.ENABLE_CUBE ? new Cube(gl) : null;
	const starburst = Settings.ENABLE_STARBURST ? new Starburst(gl, Settings.NUM_STARBURST_SPRITES) : null;

	let sceneTranslation = Settings.INITIAL_SCENE_TRANSLATION;
	const perspectiveMatrix = mat4.perspective(
		mat4.create(),
		glMatrix.toRadian(Settings.FOVY),
		gl.viewportWidth/gl.viewportHeight,
		0.1,
		100.0
	);

	gl.clearColor(0.1, 0.2, 0.3, 1.0);
	gl.enable(gl.DEPTH_TEST);
	gl.disable(gl.BLEND);

	document.onkeydown = function(event) {
		const key = event.keyCode;

		// TODO: try to get smoother scrolling using the keydown method from
		// lesson 6
		switch(key) {
			case 27: // escape key
				sceneTranslation = Settings.INITIAL_SCENE_TRANSLATION;
				break;
			case 65: // a
				sceneTranslation.x += Settings.SCENE_TRANSLATION_STEP;
				break;
			case 68: // d
				sceneTranslation.x -= Settings.SCENE_TRANSLATION_STEP;
				break;
			case 83: // s
				sceneTranslation.z -= Settings.SCENE_TRANSLATION_STEP;
				break;
			case 87: // w
				sceneTranslation.z += Settings.SCENE_TRANSLATION_STEP;
				break;
		};
	};

	let t = 0;
	let t_last_report = 0;
	let timer_start_time_ms = Date.now();
	(function drawScene() {
		window.requestAnimationFrame(drawScene);

		gl.viewport(0, 0, gl.viewportWidth, gl.viewportHeight);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

		if(Settings.ENABLE_STARBURST) {
			starburst.draw(
				perspectiveMatrix,
				t,
				sceneTranslation
			);
		}

		if(Settings.ENABLE_PYRAMID) {
			pyramid.draw(
				perspectiveMatrix,
				t,
				sceneTranslation
			);
		}

		if(Settings.ENABLE_CUBE) {
			cube.draw(
				perspectiveMatrix,
				t,
				sceneTranslation
			);
		}

		t++;

		// fps counter disabled to avoid spamming the console
		// if((t - t_last_report) == FRAMES_PER_FPS_REPORT) {
		// 	var now = Date.now();
		// 	var elapsed_ms = now - timer_start_time_ms;
		// 	var avg_render_time_ms = elapsed_ms/FRAMES_PER_FPS_REPORT;
		// 	var fps = 1000.0/avg_render_time_ms;
		// 	console.log("avg render time: " + avg_render_time_ms + " ms (last " + FRAMES_PER_FPS_REPORT + " frames), " + fps + " fps");
		// 	t_last_report = t;
		// 	timer_start_time_ms = now;
		// }
	})();
}

document.addEventListener('DOMContentLoaded', main);
