import init, {
  get_grid_sample,
  get_snake_path,
  init_panic_hook,
  IPoint,
  log,
} from "snk-js";
import wasmUrl from "snk-js/snk_js_bg.wasm";
import { createCanvas } from "../utils/canvas";

await init(wasmUrl);
init_panic_hook();
log();

{
  const grid = get_grid_sample("caves");
  const { canvas, draw, highlightCell } = createCanvas(grid);
  draw(
    { width: grid.width, height: grid.height, data: grid.cells },
    [] as any,
    [],
  );
  document.body.appendChild(canvas);

  {
    const directions = get_snake_path(
      grid,
      [
        IPoint.create(0, 0),
        IPoint.create(1, 0),
        IPoint.create(2, 0),
        IPoint.create(3, 0),
      ],
      IPoint.create(0, 5),
    );
    console.log(directions?.map(({ x, y }) => ({ x, y })));
    if (directions) {
      let px = 0;
      let py = 0;

      for (const d of directions) {
        px += d.x;
        py += d.y;
        highlightCell(px, py);
      }
    }
  }
}
