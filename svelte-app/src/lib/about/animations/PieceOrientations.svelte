<script lang="ts">
	import Piece from '../Piece.svelte';
	import { zig_zag_2x4_big } from '../piece';
	import type { PieceInfo } from '../piece';

	// Input properties
	const piece: PieceInfo = zig_zag_2x4_big;
	const width: number = piece.width;
	const height: number = piece.height;
	const hypotenuse: number = Math.sqrt(width ** 2 + height ** 2);
	const padding: number = piece.padding;
	const draw_scale: number = piece.draw_scale;
	const spacing: number = 1;
	// Calculated properties
	const viewBox_size: number = hypotenuse + padding * 2 + spacing;

	const duration: number = 15; // duration of animation in seconds

	/**
	 * Stage 0: start  = 0
	 * Stage 1: 0
	 * Stage 2: 90
	 * Stage 3: 90
	 * Stage 4: 180
	 * Stage 5: 180
	 * Stage 6: 270
	 * Stage 7: 270
	 * Stage 8: flip + 270
	 * Stage 9: flip + 270
	 * Stage 10: flip + 180
	 * Stage 11: flip + 180
	 * Stage 12: flip + 90
	 * Stage 13: flip + 90
	 * Stage 14: flip + 0
	 * Stage 15: flip + 0
	 * Stage 16: 0
	 * Stage 17: end = 0
	 */
	const stages: number = 17;
	const transition_weights: number[] = [
		2, // pause
		1, // rotate
		1, // pause
		1, // rotate
		1, // pause
		1, // rotate
		1, // pause
		1, // flip
		1, // pause
		1, // rotate
		1, // pause
		1, // rotate
		1, // pause
		1, // rotate
		1, // pause
		1, // flip
		1 // end
	];

	const rotate_spline: string = [
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1',
		'0.42 0 0.58 1',
		'0 0 1 1'
	].join(';');

	const flip_spline: string = [
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
		let values: number[] = [0, 0, 90, 90, 180, 180, 270, 270, 180, 180, 90, 90, 0, 0];

		return values.map(String).join(';');
	}

	function get_flip_values(): string {
		let values: string[] = new Array();

		values.push('1 1');
		values.push('1 1');
		values.push('1 -1');
		values.push('1 -1');
		values.push('1 1');
		values.push('1 1');

		return values.join(';');
	}
</script>

<div class="content">
	<svg width="50%" viewBox="0 0 {viewBox_size * draw_scale} {viewBox_size * draw_scale}">
		<g
			transform={`translate(
            ${(viewBox_size / 2 - width / 2 - padding) * draw_scale} 
            ${(viewBox_size / 2 - height / 2 - padding) * draw_scale}
            )`}
		>
			<Piece {piece}>
				<!-- Rotation animation -->
				<animateTransform
					attributeName="transform"
					dur={`${duration}s`}
					type="rotate"
					values={get_rotate_values()}
					keyTimes={get_keyTimes([1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 13, 14])}
					calcMode="spline"
					keySplines={rotate_spline}
					repeatCount="indefinite"
					additive="sum"
				/>
				<animateTransform
					attributeName="transform"
					dur={`${duration}s`}
					type="scale"
					values={get_flip_values()}
					keyTimes={get_keyTimes([7, 8, 15, 16])}
					calcMode="spline"
					keySplines={flip_spline}
					repeatCount="indefinite"
					additive="sum"
				/>
			</Piece>
		</g>
	</svg>
</div>

<style>
	div.content {
		display: flex;
		align-items: center;
		justify-content: center;
		margin: auto;
	}
</style>
