<script lang="ts">
	// Controls 2x3 No Hole piece
	import { getContext, onMount } from 'svelte';
	import {
		drawScale,
		lineWidth,
		Point,
		rotationAboutPoint,
		translateAndRotate,
		getStrokeLightness
	} from '$lib/game/store';

	// Export values of svelte componenet act as inputs
	export let position: Point;
	export let width: number;
	export let height: number;
	export let svgPath: string;
	export let hsl: [number, number, number];

	let isSelected: boolean = false;

	let fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	let stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;

	// the rotation angle, clockwise in radians
	let theta: number = 0; // degrees
	let centre: Point;
	let positionOffset: Point;

	const { register, unregister } = getContext('canvas');

	/**
	 * Re-calculates the centre position and sets the centre variable
	 * of the piece when called.
	 */
	function updateCentrePosition(): void {
		centre = new Point(
			position.getX() + (width * $drawScale) / 2,
			position.getY() + (height * $drawScale) / 2
		);
	}

	/**
	 * Returns the SVG moveto command to be prepended to the svgPath
	 */
	function getMoveTo(): string {
		return 'M' + position.getX() + ',' + position.getY();
	}

	/**
	 * Creates the path of the puzzle piece to be drawn
	 * @param ctx the canvas context which to draw the path on
	 */
	function drawPiece(ctx: CanvasRenderingContext2D): void {
		let ctxPath: Path2D = new Path2D(getMoveTo() + svgPath);

		// Draw the puzzle piece shape
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.stroke(ctxPath);
		ctx.fill(ctxPath);

		// Draw the square borders of the shape
		ctx.beginPath();
		ctx.strokeStyle = stroke;
		ctx.lineWidth = $lineWidth * $drawScale;

		// Draw Vertical Lines
		for (let unit: number = 0; unit < width + 1; unit++) {
			ctx.moveTo(
				position.getX() + unit * $drawScale,
				position.getY() - ($lineWidth / 2) * $drawScale
			);
			ctx.lineTo(
				position.getX() + unit * $drawScale,
				position.getY() + (height + $lineWidth / 2) * $drawScale
			);
		}

		// Draw Horizontal Lines
		for (let unit: number = 0; unit < height + 1; unit++) {
			ctx.moveTo(
				position.getX() - ($lineWidth / 2) * $drawScale,
				position.getY() + unit * $drawScale
			);
			ctx.lineTo(
				position.getX() + (width - $lineWidth / 2) * $drawScale,
				position.getY() + unit * $drawScale
			);
		}

		ctx.stroke();

		// Add canvas position on piece
		ctx.font = '30px Arial';
		ctx.fillStyle = 'black';
		ctx.fillText(
			`x: ${position.getX().toPrecision(5)}\ty: ${position.getY().toPrecision(5)}`,
			position.getX(),
			centre.getY()
		);
	}

	function drawCircle(ctx: CanvasRenderingContext2D, position: Point, fill: string): void {
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.arc(position.getX(), position.getY(), 10, 0, 2 * Math.PI, true);
		ctx.fill();
	}

	/**
	 * Checks if the current cursor position is on the shape
	 * @param cursorPosition a Point representing the current position of the cursor on the canvas
	 * @param theta the rotation angle, clockwise in radians
	 */
	function isCursorWithinPieceBoundary(cursorPosition: Point, theta: number): boolean {
		let point1 = rotationAboutPoint(position, centre, theta);
		let point2 = rotationAboutPoint(
			new Point(position.getX() + width * $drawScale, position.getY() + height * $drawScale),
			centre,
			theta
		);

		console.log(`boundary point 1: x = ${point1.getX()}\ty = ${point1.getY()}`);
		console.log(`boundary point 2: x = ${point2.getX()}\ty = ${point2.getY()}`);

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
			xMin <= cursorPosition.getX() &&
			cursorPosition.getX() <= xMax &&
			yMin <= cursorPosition.getY() &&
			cursorPosition.getY() <= yMax
		) {
			return true;
		} else {
			return false;
		}
	}

	onMount(() => {
		updateCentrePosition();

		register(draw, select, drag, drop);

		return () => {
			unregister(draw, select, drag, drop);
		};
	});

	function draw(ctx: CanvasRenderingContext2D) {
		// drawPiece(ctx);
		// drawCircle(ctx, position, 'black');

		translateAndRotate(ctx, centre, theta);
		drawPiece(ctx);
		translateAndRotate(ctx, centre, -theta);

		let rotPoints = rotationAboutPoint(position, centre, theta);

		// drawCircle(ctx, rotPoints, 'purple');
		// drawCircle(ctx, new Point(0, 0), 'red');
	}

	function select(cursorPosition: Point): void {
		if (isCursorWithinPieceBoundary(cursorPosition, theta)) {
			isSelected = true;
			positionOffset = new Point(
				position.getX() - cursorPosition.getX(),
				position.getY() - cursorPosition.getY()
			);
		}
	}

	function drag(cursorPosition: Point): void {
		if (isSelected) {
			position = new Point(
				cursorPosition.getX() + positionOffset.getX(),
				cursorPosition.getY() + positionOffset.getY()
			);
			updateCentrePosition();
		}
	}

	function drop(): void {
		isSelected = false;
	}
</script>
