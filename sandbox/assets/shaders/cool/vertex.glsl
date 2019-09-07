#version 330 core
			
layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec4 in_color;

uniform float u_rotation;

out vec4 v_color;
out vec3 v_pos;

void main()
{
	vec3 rot_pos = vec3(
		in_pos[0]*cos(u_rotation) + in_pos[1]*sin(u_rotation),
		in_pos[1]*cos(u_rotation) - in_pos[0]*sin(u_rotation),
		0.0
	);
	gl_Position = vec4(rot_pos, 1.0);

	v_color = in_color;
	v_pos = rot_pos;
}