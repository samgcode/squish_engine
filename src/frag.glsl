#version 100

precision lowp float;

varying vec2 uv;

uniform sampler2D tex;

void main() {
  // gl_FragColor = vec4(uv, 1.0, 1.0);
  gl_FragColor = texture2D(tex, uv);
}
