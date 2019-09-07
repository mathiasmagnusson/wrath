#version 330 core
			
layout(location = 0) out vec4 color;

in vec4 v_color;
in vec3 v_pos;

void main()
{
	color = v_color * (v_pos[1] + 0.2);
}
