import { serve } from "bun";
import * as childProcess from "child_process";
import * as fs from "fs";
import * as path from "path";
import snake_exit_page from "./snake_exit/index.html";
import solver_page from "./solver/index.html";
import tunnel_page from "./tunnel/index.html";

const server = serve({
	routes: {
		"/": solver_page,
		"/solver": solver_page,
		"/tunnel": tunnel_page,
		"/snake_exit": snake_exit_page,
	},

	// Enable development mode for:
	// - Detailed error messages
	// - Hot reloading (Bun v1.2.3+ required)
	development: true,
});

console.log(`Listening on ${server.url}`);

//
// Cargo rebuild
//
{
	const build = () => {
		const wasmPackFile = path.resolve(
			__dirname,
			"../../node_modules/wasm-pack/run.js",
		);
		const cwd = path.resolve(__dirname, "../../cargo/snk-js");
		const gitIgnore = path.resolve(__dirname, "../snk-js/.gitignore");
		childProcess.execSync(
			`${wasmPackFile} build --dev --target web --out-dir ../../packages/snk-js && rm ${gitIgnore}`,
			{ cwd },
		);
	};

	let timeout: number | Timer | undefined;

	fs.watch(
		path.resolve(__dirname, "../../cargo"),
		{ recursive: true },
		(_event, filename) => {
			if (filename?.startsWith("target/")) return;
			clearTimeout(timeout);
			timeout = setTimeout(build, 60);
		},
	);

	build();
}
