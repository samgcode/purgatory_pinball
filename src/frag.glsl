#version 100

precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;

uniform sampler2D _ScreenTexture;
uniform sampler2D tex;
uniform float t;

vec3 getGradient(vec4 c1, vec4 c2, vec4 c3, vec4 c4, float value_){
	float blend1 = smoothstep(c1.w, c2.w, value_);
	float blend2 = smoothstep(c2.w, c3.w, value_);
	float blend3 = smoothstep(c3.w, c4.w, value_);
	
	vec3 
	col = mix(c1.rgb, c2.rgb, blend1);
	col = mix(col, c3.rgb, blend2);
	col = mix(col, c4.rgb, blend3);
	
	return col;
}

void main() {
  const vec4 p1 = vec4(1.0, 0.290196078431, 0.290196078431, 0.0);
  const vec4 p2 = vec4(0.509803921569, 1.0, 1.0, 0.2);
  const vec4 p3 = vec4(1.0, 0.360784313725, 0.941176470588, 0.4);
  const vec4 p4 = vec4(1.0, 0.290196078431, 0.290196078431, 1.0);

  vec4 tex = texture2D(tex, uv);
  
  float a = sqrt(tex.r * tex.g * tex.b);
  float h = mod(0.5 * t + 0.4 * uv_screen.x * uv_screen.y, 1.0);

  vec4 screen = texture2D(_ScreenTexture, uv_screen);
  vec4 tile = vec4(getGradient(p1, p2, p3, p4, h), tex.a) * a;

  gl_FragColor = tile * tex.a + screen * (1.0 - tex.a); 
}
