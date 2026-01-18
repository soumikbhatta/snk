import init, {
  get_grid_sample,
  get_snake_path,
  init_panic_hook,
  IPoint,
} from "snk-js";
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

  const res = get_snake_path(
    grid,
    [
      IPoint.create(-3, 0),
      IPoint.create(-2, 0),
      IPoint.create(-1, 0),
      IPoint.create(0, 0),
    ],
    IPoint.create(30, 4),
  );
  console.log(res);
}
