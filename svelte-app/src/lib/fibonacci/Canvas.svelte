<script lang="ts">
	import { onMount, setContext } from 'svelte';
	let canvas: HTMLCanvasElement;
	export let height: number;
	export let width: number;

	const drawFunctions: Function[] = [];

	setContext('canvas', {
		register(drawFn: Function) {
			drawFunctions.push(drawFn);
		},
		unregister(drawFn: Function) {
			drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
		}
	});

	onMount(() => {
		const ctx = canvas.getContext('2d');
		canvas.height = height;
		canvas.width = width;

		function update() {
			ctx?.clearRect(0, 0, canvas.width, canvas.height);
			ctx?.save();
			ctx?.translate(canvas.width / 2, canvas.height / 2);
			drawFunctions.forEach((drawFn) => {
				drawFn(ctx);
			});

			ctx?.restore();

			frameId = requestAnimationFrame(update);
		}

		// Starts animation loop when canvas is mounted
		let frameId = requestAnimationFrame(update);

		// When canvas is demounted, onMount functions ends and loop is cancelled.
		return () => {
			cancelAnimationFrame(frameId);
		};
	});
</script>

<canvas bind:this={canvas} />

<slot />
