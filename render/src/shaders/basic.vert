#version 330

uniform mat4 mvp;

in vec3 position;
in vec4 color;

out vec4 fragColor;

void main()
{
  fragColor = color;
  gl_Position = mvp * vec4(5.0 * position, 1.0);
}