#type vertex
#version 330 core

layout(location = 0) in vec3 in_pos;

uniform uint u_x;
uniform uint u_y;
uniform uint u_cols;
uniform uint u_rows;

float map(float val, float in_min, float in_max, float out_min, float out_max) {
	return out_min + (out_max - out_min) * (val - in_min) / (in_max - in_min);
}

void main() {
	float x = map(u_x, 0.0, float(u_cols), -1.0, 1.0);
	float y = map(u_y, 0.0, float(u_rows), -1.0, 1.0);

	gl_Position = vec4(
		x + in_pos[0] / float(u_cols) * 2,
		y + in_pos[1] / float(u_rows) * 2,
		0.0, 1.0
	);
}

#type fragment
#version 330 core

layout(location = 0) out vec4 out_color;

uniform vec4 u_color;

void main() {
	out_color = u_color;
}
