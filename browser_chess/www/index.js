import {setup} from "browser_chess";

const canvas = document.getElementById('canvas');
const white_select = document.getElementById('white');
const black_select = document.getElementById('black');
const play_stop = document.getElementById('play_stop');
const reset = document.getElementById('reset');
const status = document.getElementById('status');
const descriptions = document.getElementById('descriptions');
const speed = document.getElementById('speed');

var game = setup(canvas, status, white_select, black_select, descriptions);

reset.addEventListener("click", (event) => {
    handleReset();
})

play_stop.addEventListener("click", (event) => {
    handlePlayStop()
});

speed.addEventListener("input", (event) => {
    fps = speed.value;
});

var fps = speed.value;
var playing = true;
var animation_id = null;

function handleReset() {
    game.reset();
    playing = true;
}

function handlePlayStop() {
    playing = !playing;
}


const render_loop = () => {
    setTimeout(() => {
        if (playing) {
            var done = game.loop_step();
            if (done) {
                playing = false;
            }
        } else {
            game.render();
        }
        animation_id = requestAnimationFrame(render_loop);
    }, 1000/fps)
}

render_loop();
