
#version 140

uniform mat4 trsf_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;

in vec3 position;
in vec3 color;

out vec3 vColor;

void main() {
		gl_Position = vec4(position, 1.0);//(proj_mat*view_mat*trsf_mat) * vec4(position, 1.0) ;
		vColor = color;
}
