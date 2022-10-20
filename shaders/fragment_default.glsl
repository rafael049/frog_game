#version 140

in vec3 vColor;
in vec2 k_uv;

out vec4 f_color;


uniform sampler2D tex;


void main() {
	f_color = texture(tex, k_uv);
}
