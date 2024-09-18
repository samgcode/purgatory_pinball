#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;

uniform sampler2D _ScreenTexture;
uniform float t;

vec3 hueShift(vec3 color, float hueAdjust){
  const vec3  kRGBToYPrime = vec3 (0.299, 0.587, 0.114);
  const vec3  kRGBToI      = vec3 (0.596, -0.275, -0.321);
  const vec3  kRGBToQ      = vec3 (0.212, -0.523, 0.311);

  const vec3  kYIQToR     = vec3 (1.0, 0.956, 0.621);
  const vec3  kYIQToG     = vec3 (1.0, -0.272, -0.647);
  const vec3  kYIQToB     = vec3 (1.0, -1.107, 1.704);

  float   YPrime  = dot (color, kRGBToYPrime);
  float   I       = dot (color, kRGBToI);
  float   Q       = dot (color, kRGBToQ);
  float   hue     = atan (Q, I);
  float   chroma  = sqrt (I * I + Q * Q);

  hue += hueAdjust;

  Q = chroma * sin (hue);
  I = chroma * cos (hue);

  vec3    yIQ   = vec3 (YPrime, I, Q);

  return vec3( dot (yIQ, kYIQToR), dot (yIQ, kYIQToG), dot (yIQ, kYIQToB) );
}

void main() {
    float gradient = length(uv);

    // gl_FragColor = texture2D(tex, vec2(uv.x, 1.0-uv.y));
    vec4 tex = texture2D(_ScreenTexture, uv_screen);

    if(tex.r != 0.0 && tex.r == tex.g && tex.g == tex.b) {
      vec3 col = vec3(uv_screen.x, uv_screen.y * 0.5, 1.0);
      gl_FragColor = vec4(hueShift(col, t), 1.0); 
    } else {
      gl_FragColor = tex;
    }
}

