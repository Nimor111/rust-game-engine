#version 330 core

layout (location=0) in vec3 position;
layout (location=1) in vec4 color;

out vec4 fColor;

void main() {
  fColor = color;

  gl_Position = vec4(position, 1.0);
}

#version 330 core

in vec4 fColor;

out vec4 color;

void main() {
  color = fColor;
}
