#version 140

in vec3 vColor;
in vec2 k_uv;

out vec4 f_color;


uniform sampler2D tex;


void main() {
	vec4 tex = texture(tex, k_uv);

	if (tex.w < 0.5)
		discard;
	
	f_color = tex;
}
