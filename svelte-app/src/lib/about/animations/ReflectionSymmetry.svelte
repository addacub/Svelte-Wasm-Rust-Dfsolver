<script lang="ts">
	import Piece from '../Piece.svelte';
	import type { PieceInfo } from '../piece';
	import { no_hole_2x3_sym } from '../piece';

	// Input properties
	const piece: PieceInfo = no_hole_2x3_sym;

	// Calculated properies
	const spacing: number = 0.5;
	const mirror_height: number = piece.height * 2;
	const viewBox_width: number = piece.width * 2 + spacing * 2 + piece.padding * 2;
	const viewBox_height: number = mirror_height + piece.padding * 2;
	const stroke_dasharray: number = 0.1;
	const stroke: string = 'hsl(0, 0%, 25%)';

	const duration: number = 10; // duration of animation in seconds

	/**
	 * Stage 0: start = piece + mirror line
	 * Stage 1: piece + mirror line
	 * Stage 2: piece + mirror line + mirror image
	 * Stage 3: piece + mirror line + mirror image
	 * Stage 4: piece + mirror line
	 * Stage 5: end
	 */
	const stages: number = 7;
	const transition_weights: number[] = [
		1, // pause
		1, // fade in
		1, // pause
		1, // translate
		1 // end
	];

	const move_spline: string = ['0 0 1 1', '0.42 0 0.58 1', '0 0 1 1'].join(';');

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

	function get_translate_values(): string {
		let values: number[] = [
			0,
			0,
			-(piece.width + spacing * 2) * piece.draw_scale,
			-(piece.width + spacing * 2) * piece.draw_scale
		];

		return values.map(String).join(';');
	}

	function get_fill_values(): string {
		let values: string[] = [
			'rgba(130,136,141, 1)',
			'rgba(130,136,141, 1)',
			'rgba(130,136,141, 0)',
			'rgba(130,136,141, 0)'
		];

		return values.join(';');
	}
</script>

<div class="content">
	<svg
		width="50%"
		viewBox="0 0 {viewBox_width * piece.draw_scale} {viewBox_height * piece.draw_scale}"
	>
		<g
			transform={`translate(0 ${
				(mirror_height / 2 - no_hole_2x3_sym.height / 2) * piece.draw_scale
			})`}
		>
			<Piece piece={no_hole_2x3_sym} />
		</g>

		<line
			x1={(no_hole_2x3_sym.width + spacing + piece.stroke_width / 2) * piece.draw_scale}
			y1={piece.padding * piece.draw_scale}
			x2={(no_hole_2x3_sym.width + spacing + piece.stroke_width / 2) * piece.draw_scale}
			y2={(piece.padding + mirror_height) * piece.draw_scale}
			stroke-width={piece.stroke_width * piece.draw_scale}
			{stroke}
			stroke-dasharray={`${stroke_dasharray * piece.draw_scale}, ${
				stroke_dasharray * piece.draw_scale
			}`}
		/>

		<g
			transform={`translate(${(piece.width + spacing * 2) * piece.draw_scale} ${
				(mirror_height / 2 - piece.height / 2) * piece.draw_scale
			})`}
		>
			<Piece {piece} />
			<rect
				x={(piece.padding - piece.stroke_width) * piece.draw_scale}
				y={(piece.padding - piece.stroke_width) * piece.draw_scale}
				width={(piece.width + piece.stroke_width * 2) * piece.draw_scale}
				height={(piece.height + piece.stroke_width * 2) * piece.draw_scale}
				fill="rgba(130,136,141, 1)"
			>
				<animate
					attributeName="fill"
					dur="{duration}s"
					values={get_fill_values()}
					keyTimes={get_keyTimes([1, 2])}
					repeatCount="indefinite"
				/>
			</rect>
			<animateTransform
				attributeName="transform"
				dur="{duration}s"
				type="translate"
				values={get_translate_values()}
				keyTimes={get_keyTimes([3, 4])}
				calcMode="spline"
				keySplines={move_spline}
				repeatCount="indefinite"
				additive="sum"
			/>
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
