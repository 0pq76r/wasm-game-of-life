import {Universe} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new(window.innerWidth/6.1, window.innerHeight/6.1);

document.addEventListener('keydown', (e)=>{
    universe.printLetter(Math.floor(Math.random()*10),String.fromCharCode(e.keyCode))
});

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
    //    setTimeout(()=>
    requestAnimationFrame(renderLoop)
    //        ,1000/100);
};

requestAnimationFrame(renderLoop);
