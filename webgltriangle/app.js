const vertexShaderSource = `#version 300 es

in vec2 aPosition;
in vec3 aColor;

out vec3 vColor;

void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    vColor = aColor;
}`;

const fragmentShaderSource = `#version 300 es
precision highp float;

in vec3 vColor;

out vec4 color;

void main() {
    color = vec4(vColor, 1.0);
}`;

const createShader = function (gl, shaderSource, shaderType) {
    let shader = gl.createShader(shaderType);

    gl.shaderSource(shader, shaderSource);

    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error('Error compiling shader!', gl.getShaderInfoLog(shader));
        return;
    }

    return shader;
};

const createProgram = function (gl, vertexShader, fragmentShader) {
    let program = gl.createProgram();

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.error('Error compiling program!', gl.getProgramInfoLog(program));
        return;
    }

    return program;
};

const Initialize = function () {
    console.log('This is working');

    var canvas = document.getElementById('gameCanvas');
    var gl = canvas.getContext('webgl2');

    if (!gl) {
        alert('Browser does not support WebGL! :(');
    }

    gl.clearColor(0.75, 0.85, 0.8, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    let vertexShader = createShader(gl, vertexShaderSource, gl.VERTEX_SHADER);
    let fragmentShader = createShader(gl, fragmentShaderSource, gl.FRAGMENT_SHADER);

    let program = createProgram(gl, vertexShader, fragmentShader);

    var triangleVertices = [
        0.0, 0.5, 1.0, 0.0, 0.0,
        -0.5, -0.5, 0.0, 1.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0,
    ];

    let vbo = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(triangleVertices), gl.STATIC_DRAW);

    let positionAttribLocation = gl.getAttribLocation(program, 'aPosition');
    gl.vertexAttribPointer(
        positionAttribLocation, // Position Attrib location
        2, // Elements per attribute
        gl.FLOAT, // Type of element
        gl.FALSE, // Is normalized?
        5 * Float32Array.BYTES_PER_ELEMENT,
        0
    );
    
    let colorAttribLocation = gl.getAttribLocation(program, 'aColor');
    gl.vertexAttribPointer(
        colorAttribLocation, // color Attrib location
        3, // Elements per attribute
        gl.FLOAT, // Type of element
        gl.FALSE, // Is normalized?
        5 * Float32Array.BYTES_PER_ELEMENT,
        2 * Float32Array.BYTES_PER_ELEMENT
    );

    gl.enableVertexAttribArray(positionAttribLocation);
    gl.enableVertexAttribArray(colorAttribLocation);

    gl.useProgram(program);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
};