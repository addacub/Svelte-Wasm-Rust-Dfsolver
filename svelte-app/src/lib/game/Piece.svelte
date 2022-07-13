<script lang="ts">
	// Controls 2x3 No Hole piece
	import { getContext, onMount } from 'svelte';
	import { tweened, type Tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { drawScale, lineWidth, Point, getStrokeLightness, isSelectable } from '$lib/game/store';

	// Export values of svelte componenet act as inputs
	export let position: Point;
	export let width: number;
	export let height: number;
	export let svgPath: string;
	export let hsl: [number, number, number];

	let fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	let stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;

	let isSelected: boolean = false;
	let isDragging: boolean = false;

	let centre: Point;
	let positionOffset: Point;
	// the rotation angle, clockwise in radians
	let theta: number = 0; // degrees
	let scaleHeight: number = 1; // unit length
	// Save default position
	const originalState: [position: Point, theta: number, scaleHeight: number] = [
		position,
		theta,
		scaleHeight
	];

	let tweenedTheta: Tweened<number> = tweened(theta, { duration: 350, easing: cubicOut });
	let tweenedPosition: Tweened<Point> = tweened(position, { duration: 1000, easing: cubicOut });
	let tweenedScaleHeight: Tweened<number> = tweened(scaleHeight, {
		duration: 350,
		easing: cubicOut
	});

	const { register, unregister } = getContext('canvas');

	onMount(() => {
		register(draw, select, drag, drop, rotate, flip, reset);

		return () => {
			unregister(draw, select, drag, drop, rotate, flip, reset);
		};
	});

	/**
	 * Returns a point rotated around a provided point at the specified angle.
	 * @param position The cartesian point to be rotated
	 * @param origin The point which to rotate around
	 * @param theta The angle of rotation in degrees as per standard unit circle.
	 * @returns
	 */
	export function rotationAboutPoint(position: Point, origin: Point, theta: number): Point {
		// Convert from degrees to radians
		theta = (Math.PI / 180) * theta;

		// Translate so point of rotation is the origin
		let xTranslated = position.getX() - origin.getX();
		let yTranslated = position.getY() - origin.getY();

		// Rotate
		let xRotated = xTranslated * Math.cos(theta) - yTranslated * Math.sin(theta);
		let yRotated = yTranslated * Math.cos(theta) + xTranslated * Math.sin(theta);

		// Undo translation
		let x = xRotated + origin.getX();
		let y = yRotated + origin.getY();

		return new Point(x, y);
	}

	/**
	 * Rotates and translates the canvas by the angle specified
	 * @param theta the rotation angle, clockwise in radians
	 */
	export function transformCanvas(
		ctx: CanvasRenderingContext2D,
		centre: Point,
		theta: number,
		scaleHeight: number
	): void {
		ctx.translate(centre.getX(), centre.getY());
		ctx.rotate((Math.PI / 180) * theta);
		ctx.scale(1, scaleHeight);
		ctx.translate(-centre.getX(), -centre.getY());
	}

	/**
	 * Calculates and sets a new centre position from the input reference position.
	 * @param position Reference position to calculate centre position from.
	 * Usually refers to the top left-hand corner of a shape.
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
	 * Draws the puzzle piece and applys the required transformations to the canvas to do so
	 * @param ctx the canvas context which to draw the path on
	 */
	function draw(ctx: CanvasRenderingContext2D) {
		let theta: number = $tweenedTheta;
		position = new Point((<any>$tweenedPosition).x, (<any>$tweenedPosition).y);
		let scaleHeight: number = $tweenedScaleHeight;
		updateCentrePosition();

		transformCanvas(ctx, centre, theta, scaleHeight);
		drawPiece(ctx);
		ctx.restore();
	}

	function drawCircles(ctx: CanvasRenderingContext2D): void {
		ctx.beginPath();
		ctx.fillStyle = 'red';
		ctx.arc(position.getX(), position.getY(), 0.1 * $drawScale, 0, Math.PI * 2);
		ctx.fill();

		ctx.beginPath();
		ctx.fillStyle = 'black';
		ctx.arc(centre.getX(), centre.getY(), 0.1 * $drawScale, 0, Math.PI * 2);
		ctx.fill();
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

		// Add highlight if piece is selected
		if (isSelected) {
			ctx.strokeStyle = 'rgb(253, 252, 151)';
		}

		ctx.stroke();

		// Add canvas position on piece
		ctx.font = '20px Arial';
		ctx.fillStyle = 'black';
		ctx.fillText(
			`x: ${position.getX().toPrecision(5)}\ty: ${position.getY().toPrecision(5)}`,
			position.getX(),
			centre.getY()
		);

		drawCircles(ctx);
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

	/**
	 * Determines if the mousedown event occured while the mouse was
	 * on the puzzle piece.
	 * @param cursorPosition A Point which contains the relative canvas coordinates of
	 * the cursor position.
	 */
	function select(cursorPosition: Point): void {
		// Reset if click occurs on canvas
		isSelected = false;

		if (isCursorWithinPieceBoundary(cursorPosition, theta) && $isSelectable) {
			isSelected = true;
			isSelectable.set(false);
			isDragging = true;
			positionOffset = new Point(
				position.getX() - cursorPosition.getX(),
				position.getY() - cursorPosition.getY()
			);
		}
	}

	/**
	 * Drags the selected puzzle piece to where the
	 * user takes the mouse.
	 * @param cursorPosition A Point which contains the relative canvas coordinates of
	 * the cursor position.
	 */
	function drag(cursorPosition: Point): void {
		if (isDragging) {
			position = new Point(
				cursorPosition.getX() + positionOffset.getX(),
				cursorPosition.getY() + positionOffset.getY()
			);
			tweenedPosition.set(position, { duration: 0 });
		}
	}

	/**
	 * If is partly on a board position, will cause puzzle piece
	 * to snap to it. Otherwise will stay where the user released
	 * the mouse.
	 */
	function drop(): void {
		isDragging = false;

		if (false) {
			let transformedPosition = rotationAboutPoint(position, centre, theta);

			let snapPosition = new Point(
				Math.floor(transformedPosition.getX() / $drawScale) * $drawScale,
				Math.floor(transformedPosition.getY() / $drawScale) * $drawScale
			);
			console.log('Normal');
			console.log(position);
			console.log('Transformed');
			console.log(transformedPosition);
			tweenedPosition.set(snapPosition, { duration: 250 });
		}
	}

	/**
	 * Rotates the puzzle piece by 90 degrees
	 * @param sign Indicates the direction of rotation.
	 */
	function rotate(sign: number): void {
		if (isSelected) {
			theta += 90 * sign;
			tweenedTheta.set(theta);
		}
	}

	function flip(): void {
		if (isSelected) {
			scaleHeight *= -1;
			tweenedScaleHeight.set(scaleHeight);
		}
	}

	/**
	 * Resets the puzzle pieces to their original positions
	 * around the board.
	 */
	function reset(): void {
		[position, theta, scaleHeight] = originalState;
		tweenedTheta.set(theta, { duration: 1000 });
		tweenedPosition.set(position);
		tweenedScaleHeight.set(scaleHeight);
	}
</script>
