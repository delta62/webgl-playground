(() => {
  let canvas = document.getElementById("canvas");
  let gl = canvas.getContext("webgl2");

  if (!gl) {
    throw new Error("webgl2 not supported");
  }

  let vShaderSource = `#version 300 es
in vec2 a_position;
uniform vec2 u_resolution;

void main() {
    vec2 zeroToOne = a_position / u_resolution;
    vec2 zeroToTwo = zeroToOne * 2.0;
    vec2 clipSpace = zeroToTwo * -1.0;

    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
}
`;

  let fShaderSource = `#version 300 es
precision highp float;

uniform vec4 u_color;

out vec4 outColor;

void main() {
    outColor = u_color;
}
`;

  let createShader = (type, source) => {
    let shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    let success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (success) return shader;

    console.error(gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
  };

  let createProgram = (vShader, fShader) => {
    let program = gl.createProgram();
    gl.attachShader(program, vShader);
    gl.attachShader(program, fShader);
    gl.linkProgram(program);

    let success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) return program;

    console.error(gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
  };

  let vShader = createShader(gl.VERTEX_SHADER, vShaderSource);
  let fShader = createShader(gl.FRAGMENT_SHADER, fShaderSource);
  let program = createProgram(vShader, fShader);

  let posAttr = gl.getAttribLocation(program, "a_position");
  let resAttr = gl.getUniformLocation(program, "u_resolution");
  let colorAttr = gl.getUniformLocation(program, "u_color");

  let posBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, posBuffer);

  let positions = [10, 20, 80, 20, 10, 30, 10, 30, 80, 20, 80, 30];
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

  let vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  gl.enableVertexAttribArray(posAttr);

  {
    let size = 2;
    let type = gl.FLOAT;
    let normalize = false;
    let stride = 0;
    let offset = 0;
    gl.vertexAttribPointer(posAttr, size, type, normalize, stride, offset);
  }

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  gl.clearColor(0, 0, 0, 0);
  gl.clear(gl.COLOR_BUFFER_BIT);

  gl.useProgram(program);
  gl.uniform2f(resAttr, gl.canvas.width, gl.canvas.height);
  gl.uniform4f(colorAttr, 0.0, 0.8, 0.6, 1.0);
  gl.bindVertexArray(vao);

  {
    let primitiveType = gl.TRIANGLES;
    let offset = 0;
    let count = 6;
    gl.drawArrays(primitiveType, offset, count);
  }
})();
