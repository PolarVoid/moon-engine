import * as wasm from "moon-engine";
const canvas = document.getElementById("canvas");
const gl = canvas.getContext('webgl2');
const app = new wasm.Application();

const FPS_LIMIT = 1000.0 / 30.0;
let lastDrawTime = -1;

function init() {
    if (!gl) {
        alert('Failed to initialize WebGL2 Context!');
        return;
    }

    app.init();
    let startTime = Date.now();
    function render() {
        window.requestAnimationFrame(render);
        let currentTime = Date.now();
        let deltaTime = currentTime - lastDrawTime;
        if (deltaTime >= FPS_LIMIT) {
            lastDrawTime = currentTime;

            if (canvas.height != window.innerHeight || canvas.width != window.innerWidth) {
                canvas.height = window.innerHeight;
                canvas.style.height = window.innerHeight;
                
                canvas.width = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }
            app.render(currentTime - startTime);
        }
    }

    render();
}

init();