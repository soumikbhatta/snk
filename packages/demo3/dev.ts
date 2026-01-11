import { type BuildOutput, serve } from "bun";
import * as childProcess from "child_process";
import * as fs from "fs";
import * as path from "path";

//
// JS
//
const Jsbuilder = (() => {
	let buildPromise: Promise<BuildOutput> | undefined;

	const build = () =>
		Bun.build({
			entrypoints: [__dirname + "/rr/index.html"],
		});

	const invalidate = () => {
		buildPromise = undefined;
	};
	const getArtifacts = async () => {
		while (true) {
			if (!buildPromise) buildPromise = build();
			const b = buildPromise;
			const output = await b;
			if (b === buildPromise) return output.outputs;
		}
	};
	return { invalidate, getArtifacts };
})();

fs.watch(__dirname + "/../", { recursive: true }, (event, filename) => {
	console.log(`Detected ${event} in ${filename}`);
	Jsbuilder.invalidate();
});

//
// Cargo
//
const cargoBuilder = (() => {
	const build = () => {
		const wasmPackFile = path.resolve(
			__dirname,
			"../../node_modules/wasm-pack/run.js",
		);
		childProcess.execSync(
			`${wasmPackFile} build --dev --target web --out-dir ../../packages/snk-js`,
			{ cwd: path.resolve(__dirname, "../../cargo/snk-js") },
		);
	};

	let timeout: number | Timer | undefined;
	const invalidate = () => {
		clearTimeout(timeout);
		timeout = setTimeout(build, 100);
	};

	return { invalidate };
})();

fs.watch(
	path.resolve(__dirname, "../../cargo"),
	{ recursive: true },
	(event, filename) => {
		if (filename?.startsWith("target/")) return;
		console.log(`Detected ${event} in ${filename}`);
		cargoBuilder.invalidate();
	},
);

cargoBuilder.invalidate();

//
// http server
//
serve({
	port: 3000,
	async fetch(request) {
		let { pathname } = new URL(request.url, "http://localhost");

		if (pathname === "/") pathname = "/index.html";

		const outputs = await Jsbuilder.getArtifacts();
		const artifact = outputs.find((output) => output.path === `.${pathname}`);

		if (!artifact) return new Response("Not Found", { status: 404 });

		return new Response(artifact);
	},
});

console.log("dev server started at", "http://localhost:3000");
