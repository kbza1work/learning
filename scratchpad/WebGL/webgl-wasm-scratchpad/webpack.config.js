var path = require("path");
var webpack = require("webpack");

module.exports = {
	context: path.resolve(__dirname, "."),
	entry: path.resolve(__dirname, "src/index.js"),
	output: {
		path: path.resolve(__dirname, "build"),
		publicPath: "http://localhost:8080/build/",
		filename: "bundle.js"
	},
	// source maps don't appear to work with Babel
	// devtool: "#source-map",
	module: {
		rules: [
			{
				test: /\.rs$/,
				use: {
					loader: "rust-wasm-loader",
					options: {
						path: "build",
					},
				},
			},
			{
				test: /\.css$/,
				use: [
					"style-loader",
					"css-loader",
				]
			},
			{
				test: /\.js$/,
				exclude: /(node_modules)/,
				use: [
					{
						loader: "babel-loader",
						options: {
							presets: ["es2015", "es2016"],
							cacheDirectory: "",   // cache compilation output in system tmp dir
							// source maps don't appear to work with Babel
							//sourceMaps: "inline"
						},
					},
				],
			},
			{
				test: /\.glsl$/,
				use: [
					"webpack-glsl-loader",
				],
			}
		]
	},
	devServer: {
		publicPath: "/build/",
		contentBase: path.resolve(__dirname, "src"),
		inline: true,
		overlay: true
	},
	// currently the ModuleConcatenationPlugin breaks webpack dev server hot reloading
	// plugins: [
	// 	new webpack.optimize.ModuleConcatenationPlugin(),
	// ],
	// Emscripten's generated glue code requires these Node utilities
	// Emscripten's generated glue code requires these Node utilities
	externals: {
		'fs': true,
		'path': true,
	}
};
