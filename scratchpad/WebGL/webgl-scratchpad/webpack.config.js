var path = require("path");

module.exports = {
	context: path.resolve(__dirname, "app/src"),
	entry: path.resolve(__dirname, "app/src/main.js"),
	output: {
		path: path.resolve(__dirname, "build"),
		publicPath: "http://localhost:8080/app/",
		filename: "bundle.js"
	},
	// source maps don't appear to work with Babel
	// devtool: "#source-map",
	module: {
		loaders: [
			{ test: /\.css$/, loader: "style!css" },
			{
				test: /\.js$/,
				exclude: /(node_modules)/,
				loader: "babel",
				query: {
				  presets: ["es2015"]
				},
				cacheDirectory: "",   // cache compilation output in system tmp dir
				// source maps don't appear to work with Babel
				//sourceMaps: "inline"
			},
			{
				test: /\.glsl$/,
				loader: 'webpack-glsl'
			}
		]
	},
	devServer: {
		publicPath: "/",
		contentBase: "./app",
		inline: true,
		overlay: true
	}
};
