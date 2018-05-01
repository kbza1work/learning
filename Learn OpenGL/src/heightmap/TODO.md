* what is bump mapping?
* compute normals from a height map
* move the heightmap work into a dedicated rendering engine repo?
* passing the heightmap into a shader as a texture and calculating height and normals on the fly allows alterable terrain, but would require recalculating every frame
	* what about converting the heightmap to mesh on the CPU and modifying the mesh vertices themselves and passing the mesh to the shader as before?

the heightmap function should take a setting enum specifying whether it should perform normal smoothing

[bump mapping in shader](https://stackoverflow.com/questions/5281261/generating-a-normal-map-from-a-height-map)

[basic smoothed normals](http://www.lighthouse3d.com/opengl/terrain/index.php?normals)
[smoothed normals weighted by relative face angle](https://stackoverflow.com/questions/19905637/normal-averaging-of-heightmap)
