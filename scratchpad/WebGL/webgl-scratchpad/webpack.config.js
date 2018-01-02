var path = require("path");

module.exports = {
	context: path.resolve(__dirname, "src/js"),
	entry: path.resolve(__dirname, "src/js/main.js"),
	output: {
		path: path.resolve(__dirname, "build"),
		publicPath: "http://localhost:8080/app/",
		filename: "bundle.js"
	},
	// source maps don't appear to work with Babel
	// devtool: "#source-map",
	module: {
		rules: [
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
		publicPath: "/",
		contentBase: "./src",
		inline: true,
		overlay: true
	}
};
