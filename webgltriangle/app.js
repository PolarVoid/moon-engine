const vertexShaderSource = `#version 300 es

in vec3 aPosition;
in vec3 aColor;

uniform mat4 mWorld;
uniform mat4 mView;
uniform mat4 mProj;

out vec3 vColor;

void main() {
    gl_Position = mProj * mView * mWorld * vec4(aPosition, 1.0);
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

    gl.enable(gl.DEPTH_TEST);
    gl.enable(gl.CULL_FACE);
    gl.cullFace(gl.BACK);

    let vertexShader = createShader(gl, vertexShaderSource, gl.VERTEX_SHADER);
    let fragmentShader = createShader(gl, fragmentShaderSource, gl.FRAGMENT_SHADER);

    let program = createProgram(gl, vertexShader, fragmentShader);

    var triangleVertices = [
        0.0, 0.5, 0.0,      1.0, 1.0, 0.0,
        -0.5, -0.5, 0.0,    0.7, 0.0, 1.0,
        0.5, -0.5, 0.0,     0.1, 1.0, 0.6,
    ];

    var boxVertices = [
        // Top
        -1.0, 1.0, -1.0,   0.5, 0.5, 0.5,
		-1.0, 1.0, 1.0,    0.5, 0.5, 0.5,
		1.0, 1.0, 1.0,     0.5, 0.5, 0.5,
		1.0, 1.0, -1.0,    0.5, 0.5, 0.5,

		// Left
		-1.0, 1.0, 1.0,    0.75, 0.25, 0.5,
		-1.0, -1.0, 1.0,   0.75, 0.25, 0.5,
		-1.0, -1.0, -1.0,  0.75, 0.25, 0.5,
		-1.0, 1.0, -1.0,   0.75, 0.25, 0.5,

		// Right
		1.0, 1.0, 1.0,    0.25, 0.25, 0.75,
		1.0, -1.0, 1.0,   0.25, 0.25, 0.75,
		1.0, -1.0, -1.0,  0.25, 0.25, 0.75,
		1.0, 1.0, -1.0,   0.25, 0.25, 0.75,

		// Front
		1.0, 1.0, 1.0,    1.0, 0.0, 0.15,
		1.0, -1.0, 1.0,    1.0, 0.0, 0.15,
		-1.0, -1.0, 1.0,    1.0, 0.0, 0.15,
		-1.0, 1.0, 1.0,    1.0, 0.0, 0.15,

		// Back
		1.0, 1.0, -1.0,    0.0, 1.0, 0.15,
		1.0, -1.0, -1.0,    0.0, 1.0, 0.15,
		-1.0, -1.0, -1.0,    0.0, 1.0, 0.15,
		-1.0, 1.0, -1.0,    0.0, 1.0, 0.15,

		// Bottom
		-1.0, -1.0, -1.0,   0.5, 0.5, 1.0,
		-1.0, -1.0, 1.0,    0.5, 0.5, 1.0,
		1.0, -1.0, 1.0,     0.5, 0.5, 1.0,
		1.0, -1.0, -1.0,    0.5, 0.5, 1.0,
    ];

    var boxIndices = [
		// Top
		0, 1, 2,
		0, 2, 3,

		// Left
		5, 4, 6,
		6, 4, 7,

		// Right
		8, 9, 10,
		8, 10, 11,

		// Front
		13, 12, 14,
		15, 14, 12,

		// Back
		16, 17, 18,
		16, 18, 19,

		// Bottom
		21, 20, 22,
		22, 20, 23
	];

    let vbo = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(boxVertices), gl.STATIC_DRAW);

    let ibo = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint8Array(boxIndices), gl.STATIC_DRAW);

    let positionAttribLocation = gl.getAttribLocation(program, 'aPosition');
    gl.vertexAttribPointer(
        positionAttribLocation, // Position Attrib location
        3, // Elements per attribute
        gl.FLOAT, // Type of element
        gl.FALSE, // Is normalized?
        6 * Float32Array.BYTES_PER_ELEMENT,
        0
    );
    
    let colorAttribLocation = gl.getAttribLocation(program, 'aColor');
    gl.vertexAttribPointer(
        colorAttribLocation, // Color Attrib location
        3, // Elements per attribute
        gl.FLOAT, // Type of element
        gl.FALSE, // Is normalized?
        6 * Float32Array.BYTES_PER_ELEMENT,
        3 * Float32Array.BYTES_PER_ELEMENT
    );

    gl.enableVertexAttribArray(positionAttribLocation);
    gl.enableVertexAttribArray(colorAttribLocation);

    gl.useProgram(program);

    let matWorldUniformLocation = gl.getUniformLocation(program, 'mWorld');
    let matViewUniformLocation = gl.getUniformLocation(program, 'mView');
    let matProjUniformLocation = gl.getUniformLocation(program, 'mProj');

    var identityMatrix = new Float32Array(16);
    var worldMatrix = new Float32Array(16);
    var viewMatrix = new Float32Array(16);
    var projMatrix = new Float32Array(16);

    glMatrix.mat4.identity(identityMatrix);
    glMatrix.mat4.identity(worldMatrix);
    glMatrix.mat4.lookAt(viewMatrix, [0, 2, -5], [0, 0, 0], [0, 1, 0]);
    glMatrix.mat4.perspective(projMatrix, glMatrix.glMatrix.toRadian(45), canvas.width/canvas.height, 0.1, 1000.0);

    gl.uniformMatrix4fv(matViewUniformLocation, gl.FALSE, viewMatrix);
    gl.uniformMatrix4fv(matProjUniformLocation, gl.FALSE, projMatrix);

    let angle = 0;
    var loop = function() {
        angle = performance.now() / 1000 / 6 * 2 * Math.PI;
        glMatrix.mat4.rotate(worldMatrix, identityMatrix, angle, [0, 1, 0]);
        gl.uniformMatrix4fv(matWorldUniformLocation, gl.FALSE, worldMatrix);

        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        gl.drawElements(gl.TRIANGLES, boxIndices.length, gl.UNSIGNED_BYTE, 0);

        requestAnimationFrame(loop);
    };
    requestAnimationFrame(loop);
};