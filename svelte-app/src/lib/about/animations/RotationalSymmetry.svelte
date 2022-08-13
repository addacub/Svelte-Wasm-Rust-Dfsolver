<script lang="ts">
	import Piece from '../Piece.svelte';
	import { no_hole_2x3_sym } from '../piece';
	import type { PieceInfo } from '../piece';

	// Input properties
	const piece: PieceInfo = no_hole_2x3_sym;
	const width: number = piece.width;
	const height: number = piece.height;
	const hypotenuse: number = Math.sqrt(width ** 2 + height ** 2);
	const padding: number = piece.padding;
	const draw_scale: number = piece.draw_scale;
	const spacing: number = 1;
	// Calculated properties
	const viewBox_size: number = hypotenuse + padding * 2 + spacing;
	const translate_x: number = viewBox_size / 2 - width / 2 - padding;
	const translate_y: number = viewBox_size / 2 - height / 2 - padding;
	const cx: number = viewBox_size / 2;
	const cy: number = viewBox_size / 2;
	const r1: number = 0.1;
	const r2: number = hypotenuse / 2 + spacing * 0.5;
	const stroke: string = 'hsl(0, 0%, 25%)';
	const stroke_dasharray: number = 0.1;

	const duration: number = 10; // duration of animation in seconds

	/**
	 * Stage 0: start  = 0
	 * Stage 1: 0
	 * Stage 2: 180
	 * Stage 3: 180
	 * Stage 4: 360
	 * Stage 5: end = 360
	 */
	const stages: number = 15;
	const transition_weights: number[] = [
		1, // pause
		3, // rotate
		2, // pause
		3, // rotate
		1 // end
	];

	const rotate_spline: string = [
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1'
	].join(';');

	const frames: number = transition_weights.reduce((accumulator, current) => {
		return accumulator + current;
	}, 0);
	const increment: number = 1 / frames;

	function get_keyTimes(stages: number[]): string {
		let key_times: number[] = [0];

		let t0: number = 0;
		stages.forEach((t1, index) => {
			key_times.push(
				transition_weights.slice(t0, t1).reduce((accumulator, current) => {
					return accumulator + current;
				}, 0) *
					increment +
					key_times[index]
			);
			t0 = t1;
		});

		key_times.push(1);
		return key_times.map(String).join(';');
	}

	function get_rotate_values(): string {
		let values: number[] = [0, 0, 180, 180, 360, 360];

		return values.map(String).join(';');
	}
</script>

<div class="content">
	<svg width={viewBox_size * draw_scale} height={viewBox_size * draw_scale}>
		<rect
			x={(cx - width / 2) * draw_scale}
			y={(cy - height / 2) * draw_scale}
			width={width * draw_scale}
			height={height * draw_scale}
			fill="none"
			stroke="green"
			stroke-width={piece.stroke_width * draw_scale}
			stroke-dasharray={`${stroke_dasharray * piece.draw_scale}, ${
				stroke_dasharray * piece.draw_scale
			}`}
		/>
		<g transform-origin={`${cx * draw_scale} ${cy * draw_scale}`}>
			<g
				transform={`translate(
            ${translate_x * draw_scale} 
            ${translate_y * draw_scale}
            )`}
			>
				<Piece {piece}>
					<!-- Rotation animation -->
				</Piece>
			</g>
			<circle
				cx={cx * draw_scale}
				cy={cy * draw_scale}
				r={r1 * draw_scale}
				{stroke}
				fill={stroke}
			/>
			<circle
				cx={cx * draw_scale}
				cy={cy * draw_scale}
				r={r2 * draw_scale}
				{stroke}
				fill="none"
				stroke-width={piece.stroke_width * draw_scale}
			/>
			<line
				x1={cx * draw_scale}
				y1={(cy - r2) * draw_scale}
				x2={cx * draw_scale}
				y2={(cy - r2 + r1 * 4) * draw_scale}
				{stroke}
				stroke-width={piece.stroke_width * draw_scale}
			/>
			<animateTransform
				attributeName="transform"
				dur={`${duration}s`}
				type="rotate"
				values={get_rotate_values()}
				keyTimes={get_keyTimes([1, 2, 3, 4])}
				calcMode="spline"
				keySplines={rotate_spline}
				repeatCount="indefinite"
				additive="sum"
			/>
		</g>
	</svg>
</div>

<style>
	div.content {
		display: flex;
		margin: auto;
	}
</style>
