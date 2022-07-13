<script lang="ts">
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

	// Styling variables
	let fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	let stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;

	// Flags
	let isSelected: boolean = false;
	let isDragging: boolean = false;

	// Position
	let centre: Point;
	let tweenedCentre: Point;
	let positionOffset: Point;

	// Transformation inputs
	// the rotation angle, clockwise in radians
	let thetaDeg: number = 0; // degrees
	let scaleHeight: number = 1; // unit length

	// Save default position
	const originalState: [position: Point, theta: number, scaleHeight: number] = [
		position,
		thetaDeg,
		scaleHeight
	];

	// Set up tweened values
	let tweenedTheta: Tweened<number> = tweened(thetaDeg, { duration: 350, easing: cubicOut });
	let tweenedPosition: Tweened<Point> = tweened(position, {
		duration: 1000,
		easing: cubicOut,
		interpolate: (a: Point, b: Point) => (t: number) => {
			return new Point(a.getX() * (1 - t) + b.getX() * t, a.getY() * (1 - t) + b.getY() * t);
		}
	});
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
	 * @param centreOfRotation The point which to rotate around
	 * @param theta The angle of rotation in degrees as per standard unit circle.
	 * @returns
	 */
	export function getTransformedPosition(
		position: Point,
		centreOfRotation: Point,
		thetaDeg: number,
		scaleHeight: number
	): Point {
		// Convert from degrees to radians
		let thetaRad = (Math.PI / 180) * thetaDeg;

		// Translate so point of rotation is the origin
		let xTranslated: number = position.getX() - centreOfRotation.getX();
		let yTranslated: number = position.getY() - centreOfRotation.getY();

		// Rotate
		let yRotated: number = yTranslated * Math.cos(thetaRad) + xTranslated * Math.sin(thetaRad);
		let xRotated: number = xTranslated * Math.cos(thetaRad) - yTranslated * Math.sin(thetaRad);

		// Flip
		yRotated *= scaleHeight;

		// Undo translation
		let x: number = xRotated + centreOfRotation.getX();
		let y: number = yRotated + centreOfRotation.getY();

		return new Point(x, y);
	}

	/**
	 * Rotates and translates the canvas by the angle specified.
	 * Tweened inputs requried
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

		console.log($tweenedPosition);

		tweenedCentre = new Point(
			$tweenedPosition.getX() + (width * $drawScale) / 2,
			$tweenedPosition.getY() + (height * $drawScale) / 2
		);
	}

	/**
	 * Returns the SVG moveto command to be prepended to the svgPath.
	 * Tweened inputs required.
	 */
	function getMoveTo(position: Point): string {
		return 'M' + position.getX() + ',' + position.getY();
	}

	/**
	 * Draws the puzzle piece and applys the required transformations to the canvas to do so.
	 * Tweened inputs required.
	 * @param ctx the canvas context which to draw the path on
	 */
	function draw(ctx: CanvasRenderingContext2D) {
		updateCentrePosition();
		transformCanvas(ctx, tweenedCentre, $tweenedTheta, $tweenedScaleHeight);
		drawPiece(ctx, $tweenedPosition, tweenedCentre);
		ctx.restore();
	}

	// TODO - Can be removed when no longer needed. Tweened inputs required.
	function drawCircles(ctx: CanvasRenderingContext2D, position: Point, centre: Point): void {
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
	 * Creates the path of the puzzle piece to be drawn.
	 * Tweened inputs required. Centre argument can be removed in future
	 * @param ctx the canvas context which to draw the path on
	 */
	// TODO - remove centre argument in future
	function drawPiece(ctx: CanvasRenderingContext2D, position: Point, centre: Point): void {
		let ctxPath: Path2D = new Path2D(getMoveTo(position) + svgPath);

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

		drawCircles(ctx, position, centre);
	}

	/**
	 * Checks if the current cursor position is on the shape
	 * Do not use tweened values.
	 * @param cursorPosition a Point representing the current position of the cursor on the canvas
	 */
	function isCursorWithinPieceBoundary(cursorPosition: Point): boolean {
		let point1 = getTransformedPosition(position, centre, thetaDeg, scaleHeight);
		let point2 = getTransformedPosition(
			new Point(position.getX() + width * $drawScale, position.getY() + height * $drawScale),
			centre,
			thetaDeg,
			scaleHeight
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
	 * on the puzzle piece. Do not use tweened inputs.
	 * @param cursorPosition A Point which contains the relative canvas coordinates of
	 * the cursor position.
	 */
	function select(cursorPosition: Point): void {
		// Reset if click occurs on canvas
		isSelected = false;

		if (isCursorWithinPieceBoundary(cursorPosition) && $isSelectable) {
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
	 * Do not use tweened values.
	 */
	function drop(): void {
		isDragging = false;

		let truePosition: Point = getTransformedPosition(position, centre, thetaDeg, scaleHeight);

		let snapPosition = new Point(
			Math.floor(truePosition.getX() / $drawScale) * $drawScale,
			Math.floor(truePosition.getY() / $drawScale) * $drawScale
		);

		position = getTransformedPosition(snapPosition, centre, -thetaDeg, scaleHeight);
		tweenedPosition.set(position, { duration: 250 });
	}

	/**
	 * Rotates the puzzle piece by 90 degrees
	 * @param sign Indicates the direction of rotation.
	 */
	function rotate(sign: number): void {
		if (isSelected) {
			thetaDeg += 90 * sign;
			tweenedTheta.set(thetaDeg);
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
		[position, thetaDeg, scaleHeight] = originalState;

		tweenedTheta.set(thetaDeg, { duration: 1000 });
		tweenedPosition.set(position, {});
		tweenedScaleHeight.set(scaleHeight);
	}
</script>
