<script lang="ts">
	import { onMount, setContext, tick } from 'svelte';
	import { Point, mousePosition, scaleFactor } from './store';

	let canvas: HTMLCanvasElement;

	export let height: number;
	export let width: number;

	const drawFunctions: Function[] = [];
	const dragFunctions: Function[] = [];
	const selectFunctions: Function[] = [];
	const dropFunctions: Function[] = [];

	function mouseDown(event: MouseEvent) {
		let cursorPosition: Point = getMousePosition(event);
		selectFunctions.forEach((selectFn) => {
			selectFn(cursorPosition);
		});
	}

	function mouseUp(): void {
		dropFunctions.forEach((dropFn) => {
			dropFn();
		});
	}

	function mousemove(event: MouseEvent): void {
		let cursorPosition: Point = getMousePosition(event);
		dragFunctions.forEach((dragFn) => {
			dragFn(cursorPosition);
		});
	}

	function getMousePosition(event: MouseEvent): Point {
		let rect = canvas.getBoundingClientRect();
		// let x = (event.clientX - rect.left) / $scaleFactor;
		// let y = (event.clientY - rect.top) / $scaleFactor;

		let x = event.clientX - rect.left;
		let y = event.clientY - rect.top;

		$mousePosition = new Point(x, y);

		return new Point(x, y);
	}

	function drawBoard(ctx: CanvasRenderingContext2D, p: number): void {
		for (let x = 0; x <= canvas.width; x += 100) {
			ctx.moveTo(x + p, p);
			ctx.lineTo(x + p, canvas.height + p);
		}

		for (let y = 0; y <= canvas.height; y += 100) {
			ctx.moveTo(p, y + p);
			ctx.lineTo(canvas.width, y + p);
		}

		ctx.strokeStyle = 'black';
		ctx.stroke();
	}

	setContext('canvas', {
		register(drawFn: Function, selectFn: Function, dragFn: Function, dropFn: Function) {
			drawFunctions.push(drawFn);
			selectFunctions.push(selectFn);
			dragFunctions.push(dragFn);
			dropFunctions.push(dropFn);
		},
		unregister(drawFn: Function, selectFn: Function, dragFn: Function, dropFn: Function) {
			drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
			selectFunctions.splice(selectFunctions.indexOf(selectFn), 1);
			dragFunctions.splice(dragFunctions.indexOf(dragFn), 1);
			dropFunctions.splice(dropFunctions.indexOf(dropFn), 1);
		}
	});

	onMount(async () => {
		await tick();
		const ctx = canvas.getContext('2d')!;

		function update() {
			ctx.fillStyle = 'white';
			ctx?.rect(0, 0, width, height);
			ctx?.save();

			ctx.fill();

			drawBoard(ctx, 0);

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

<canvas
	bind:this={canvas}
	on:mousedown={mouseDown}
	on:mouseup={mouseUp}
	on:mousemove={mousemove}
	style="boarder:1px solid #000000"
/>

<slot />
