<script lang="ts">
	// Controls 2x3 No Hole piece
	import { getContext, onMount } from 'svelte';
	import {
		draw_scale,
		LINEWIDTH,
		Point,
		rotationAboutPoint,
		translateAndRotate,
		getStrokeLightness
	} from '$lib/game/store';

	export let hsl: [number, number, number];

	let fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	let stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;

	let position: Point = new Point(-400, -200);

	const WIDTH: number = 3; // units
	const HEIGHT: number = 2; // units

	let theta: number = 90; // degrees

	let centre: Point = new Point(
		position.getX() + (WIDTH * $draw_scale) / 2,
		position.getY() + (HEIGHT * $draw_scale) / 2
	);
	console.log(centre);

	const { register, unregister } = getContext('canvas');

	function drawPiece(ctx: CanvasRenderingContext2D, fill: string): void {
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.rect(position.getX(), position.getY(), WIDTH * $draw_scale, HEIGHT * $draw_scale);
		ctx.fill();

		ctx.beginPath();
		ctx.strokeStyle = stroke;
		ctx.lineWidth = $LINEWIDTH * $draw_scale;

		// Draw Vertical Lines
		for (let unit: number = 1; unit < WIDTH; unit++) {
			ctx.moveTo(position.getX() + unit * $draw_scale, position.getY());
			ctx.lineTo(position.getX() + unit * $draw_scale, position.getY() + HEIGHT * $draw_scale);
		}

		// Draw Horizontal Lines
		for (let unit: number = 1; unit < HEIGHT; unit++) {
			ctx.moveTo(position.getX(), position.getY() + unit * $draw_scale);
			ctx.lineTo(position.getX() + WIDTH * $draw_scale, position.getY() + unit * $draw_scale);
		}

		ctx.stroke();
	}

	function drawCircle(ctx: CanvasRenderingContext2D, position: Point, fill: string): void {
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.arc(position.getX(), position.getY(), 10, 0, 2 * Math.PI, true);
		ctx.fill();
	}

	function inPieceBoundary(cursor: Point, theta: number): boolean {
		let point1 = rotationAboutPoint(position, centre, theta);
		let point2 = rotationAboutPoint(
			new Point(position.getX() + WIDTH * $draw_scale, position.getY() + HEIGHT * $draw_scale),
			centre,
			theta
		);

		let xMin: number;
		let xMax: number;

		if (point1.getX() > point2.getX()) {
			xMax = point1.getX();
			xMin = point2.getX();
		} else {
			xMin = point1.getX();
			xMax = point2.getX();
		}

		let yMin: number;
		let yMax: number;

		if (point1.getY() > point2.getY()) {
			yMax = point1.getY();
			yMin = point2.getY();
		} else {
			yMin = point1.getY();
			yMax = point2.getY();
		}

		if (
			xMin <= cursor.getX() &&
			cursor.getX() <= xMax &&
			yMin <= cursor.getY() &&
			cursor.getY() <= yMax
		) {
			return true;
		} else {
			return false;
		}
	}

	function drawAsymPiece(ctx: CanvasRenderingContext2D, fill: string): void {
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.moveTo(position.getX(), position.getY());
		ctx.lineTo(position.getX() + 1 * $draw_scale, position.getY());
		ctx.lineTo(position.getX() + 1 * $draw_scale, position.getY() + 1 * $draw_scale);
		ctx.lineTo(position.getX() + 2 * $draw_scale, position.getY() + 1 * $draw_scale);
		ctx.lineTo(position.getX() + 2 * $draw_scale, position.getY());
		ctx.lineTo(position.getX() + 3 * $draw_scale, position.getY());
		ctx.lineTo(position.getX() + 3 * $draw_scale, position.getY() + 2 * $draw_scale);
		ctx.lineTo(position.getX(), position.getY() + 2 * $draw_scale);
		ctx.closePath();
		ctx.fill();
	}

	onMount(() => {
		register(draw);

		return () => {
			unregister(draw);
		};
	});

	function draw(ctx: CanvasRenderingContext2D) {
		// drawPiece(ctx, fill);
		drawAsymPiece(ctx, 'red');
		drawCircle(ctx, position, 'black');

		// Translation & rotation
		translateAndRotate(ctx, centre, theta);

		// drawPiece(ctx, 'blue');
		drawAsymPiece(ctx, 'blue');

		// Translation & rotation
		translateAndRotate(ctx, centre, -theta);

		let rotPoints = rotationAboutPoint(position, centre, theta);

		drawCircle(ctx, rotPoints, 'purple');
		drawCircle(ctx, new Point(0, 0), 'red');

		console.log(inPieceBoundary(new Point(-300, 50), theta));
	}
</script>
