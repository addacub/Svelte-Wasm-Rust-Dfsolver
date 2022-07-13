<script lang="ts">
	import { onMount, setContext, tick } from 'svelte';
	import { Point, mousePosition, drawScale, isSelectable } from './store';

	let canvas: HTMLCanvasElement;

	export let height: number;
	export let width: number;

	const drawFunctions: Function[] = [];
	const dragFunctions: Function[] = [];
	const selectFunctions: Function[] = [];
	const dropFunctions: Function[] = [];
	const rotateFunctions: Function[] = [];
	const flipFunctions: Function[] = [];
	const resetFunctions: Function[] = [];

	function mouseDown(event: MouseEvent) {
		isSelectable.set(true);
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
		let x = event.clientX - rect.left;
		let y = event.clientY - rect.top;

		$mousePosition = new Point(x, y);

		return new Point(x, y);
	}

	function rotatePiece(sign: number): void {
		rotateFunctions.forEach((rotateFn) => {
			rotateFn(sign);
		});
	}

	function flipPiece(): void {
		flipFunctions.forEach((flipFn) => {
			flipFn();
		});
	}

	function resetPieces(): void {
		resetFunctions.forEach((resetFn) => {
			resetFn();
		});
	}

	function handleKeydown(event: KeyboardEvent): void {
		let key: string = event.key;

		if (key === 'w' || key === 's') {
			flipPiece();
		}

		if (key === 'a') {
			rotatePiece(-1);
		}

		if (key === 'd') {
			rotatePiece(1);
		}

		if (key === 'Escape') {
			resetPieces();
		}
	}

	function drawBoard(ctx: CanvasRenderingContext2D, p: number): void {
		for (let x = 0; x <= canvas.width; x += $drawScale) {
			ctx.moveTo(x + p, p);
			ctx.lineTo(x + p, canvas.height + p);
		}

		for (let y = 0; y <= canvas.height; y += $drawScale) {
			ctx.moveTo(p, y + p);
			ctx.lineTo(canvas.width, y + p);
		}

		ctx.strokeStyle = 'black';
		ctx.stroke();
	}

	setContext('canvas', {
		register(
			drawFn: Function,
			selectFn: Function,
			dragFn: Function,
			dropFn: Function,
			rotateFn: Function,
			flipFn: Function,
			resetFn: Function
		) {
			drawFunctions.push(drawFn);
			selectFunctions.push(selectFn);
			dragFunctions.push(dragFn);
			dropFunctions.push(dropFn);
			rotateFunctions.push(rotateFn);
			flipFunctions.push(flipFn);
			resetFunctions.push(resetFn);
		},
		unregister(
			drawFn: Function,
			selectFn: Function,
			dragFn: Function,
			dropFn: Function,
			rotateFn: Function,
			flipFn: Function,
			resetFn: Function
		) {
			drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
			selectFunctions.splice(selectFunctions.indexOf(selectFn), 1);
			dragFunctions.splice(dragFunctions.indexOf(dragFn), 1);
			dropFunctions.splice(dropFunctions.indexOf(dropFn), 1);
			rotateFunctions.splice(rotateFunctions.indexOf(rotateFn), 1);
			flipFunctions.splice(flipFunctions.indexOf(flipFn), 1);
			resetFunctions.splice(resetFunctions.indexOf(resetFn), 1);
		}
	});

	function resizeCanvas(): void {
		canvas.width = width;
		canvas.height = height;
	}

	onMount(async () => {
		await tick();
		const ctx = canvas.getContext('2d')!;

		function update() {
			resizeCanvas();
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

<svelte:window on:keydown={handleKeydown} />

<canvas
	bind:this={canvas}
	on:mousedown={mouseDown}
	on:mouseup={mouseUp}
	on:mousemove={mousemove}
	style="boarder:1px solid #000000"
/>

<div>
	<button id="rotateAnticlockwise" on:click={() => rotatePiece(-1)}>&cularr;</button>
	<button id="rotateClockwise" on:click={() => rotatePiece(1)}>&curarr;</button>
	<button id="flip" on:click={() => flipPiece()}>&rarrlp;</button>
	<button id="reset" on:click={() => resetPieces()}>&#10227;</button>
</div>

<slot />

<style>
	div {
		padding-top: 1rem;
	}

	button {
		background-color: #c8adc0;
		width: 2em;
		height: 2em;
		font-size: xx-large;
		border-width: 5px;
		border-color: white rgb(110, 110, 110) rgb(110, 110, 110) white;
	}

	button:active {
		border-color: rgb(110, 110, 110) white white rgb(110, 110, 110);
	}

	button:hover {
		background-color: hsl(318, 20%, 50%);
	}
</style>
