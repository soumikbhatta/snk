import init, { get_grid_sample, init_panic_hook } from "snk-js";
import wasmUrl from "snk-js/snk_js_bg.wasm";
import { createCanvas } from "../utils/canvas";

await init(wasmUrl);
init_panic_hook();

{
	const grid = get_grid_sample("caves");
	const { canvas, draw } = createCanvas(grid);
	draw(
		{ width: grid.width, height: grid.height, data: grid.cells },
		[] as any,
		[],
	);
	document.body.appendChild(canvas);
}
