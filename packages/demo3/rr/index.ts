// auto-refresh
fetch("/")
	.then((res) => res.text())
	.then(async (original) => {
		while (true) {
			await new Promise((resolve) => setTimeout(resolve, 1000));
			const newContent = await fetch("/").then((res) => res.text());
			if (newContent !== original) {
				window.location.reload();
			}
		}
	});

import init, { get_grid_sample, init_panic_hook } from "snk-js";
import wasmUrl from "snk-js/snk_js_bg.wasm";
import { createCanvas } from "../utils/canvas";

await init(wasmUrl);
init_panic_hook();

{
	const grid = get_grid_sample("caves");
	const { canvas, draw } = createCanvas(grid);
	draw({ width: grid.width, height: grid.height, data: grid.cells }, [], []);
	document.body.appendChild(canvas);
}
