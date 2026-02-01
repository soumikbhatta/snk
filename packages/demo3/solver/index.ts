import init, {
	get_grid_sample,
	IPoint,
	init_log,
	init_panic_hook,
	solve,
} from "snk-js";
import wasmUrl from "snk-js/snk_js_bg.wasm";
import { createCanvas } from "../utils/canvas";

await init(wasmUrl);
init_panic_hook();
init_log();

const grid = get_grid_sample("cave2");
const { canvas, draw, highlightCell } = createCanvas(grid);
document.body.appendChild(canvas);

const path = solve(grid, [
	IPoint.create(0, -1),
	IPoint.create(1, -1),
	IPoint.create(2, -1),
	IPoint.create(3, -1),
]);

draw(
	{ width: grid.width, height: grid.height, data: grid.cells },
	[] as any,
	[],
);

console.log(path);

for (const { x, y } of path) highlightCell(x, y);
