#version 330

uniform mat4 mvp;

in vec3 pos;
in vec4 color;

out vec4 fragColor;

void main()
{
  fragColor = color;
  gl_Position = mvp * vec4(pos, 1.0);
}