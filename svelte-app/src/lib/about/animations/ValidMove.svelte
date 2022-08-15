<script lang="ts">
	import Board from '$lib/about/Board.svelte';
	import { board } from '$lib/about/board';
	import Piece from '../Piece.svelte';
	import { end_hole_2x3_big, middle_hole_2x3_big } from '../piece';

	// Input properties
	const stroke_width: number = board.stroke_width;
	const padding: number = board.padding;
	const draw_scale: number = board.draw_scale;
	const spacing: number = 2;
	const viewBox_width: number = board.width + middle_hole_2x3_big.width + padding * 2 + spacing;
	const viewBox_height: number = board.height + padding * 2;

	const duration: number = 10; // duration of animation in seconds

	const highlight: [empty: string, showing: string] = [
		'rgba(253, 252, 151, 0)',
		'rgba(253, 252, 151, 1)'
	];

	const move_spline: string = ['0 0 1 1', '0.42 0 0.58 1', '0 0 1 1'].join(';');

	/**
	 * Stage 0: start = board-hightlight; piece_side-highlight
	 * Stage 1: board-hightlight; piece_side-highlight
	 * Stage 2: board + piece-highlight
	 * Stage 3: board + piece-highlight
	 * Stage 4: board + piece
	 * Stage 5: board + piece
	 * Stage 6: board-highlight + piece
	 * Stage 7: end = board-highlight + piece
	 */
	const stages: number = 7;
	const transition_weights: number[] = [
		3, // pause
		3, // move
		0.5, // pause
		0.5, // fade out
		0.5, // pause
		0.5, // fade in
		3 // pause
	];

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

	function get_move_values(): string {
		let values: string[] = new Array();

		values.push(((board.width + spacing) * draw_scale).toString());
		values.push(((board.width + spacing) * draw_scale).toString());
		values.push((3 * draw_scale).toString());
		values.push((3 * draw_scale).toString());

		return values.join(';');
	}

	function get_fade_out_values(): string {
		let values: string[] = new Array();

		values.push(highlight[1]);
		values.push(highlight[1]);
		values.push(highlight[0]);
		values.push(highlight[0]);

		return values.join(';');
	}

	function get_fade_in_values(): string {
		let values: string[] = new Array();

		values.push(highlight[0]);
		values.push(highlight[0]);
		values.push(highlight[1]);
		values.push(highlight[1]);

		return values.join(';');
	}
</script>

<div class="content">
	<svg width="90%" viewBox="0 0 {viewBox_width * draw_scale} {viewBox_height * draw_scale}">
		<!-- Board -->
		<Board {board}>
			<!-- Board Highlight -->
			<rect
				x={(padding + stroke_width / 2 + 3) * draw_scale}
				y={(padding + stroke_width / 2) * draw_scale}
				width={(1 - stroke_width) * draw_scale}
				height={(1 - stroke_width) * draw_scale}
				fill={highlight[1]}
			/>

			<rect
				x={(padding + stroke_width / 2 + 4) * draw_scale}
				y={(padding + stroke_width / 2) * draw_scale}
				width={(1 - stroke_width) * draw_scale}
				height={(1 - stroke_width) * draw_scale}
			>
				<animate
					attributeName="fill"
					dur={`${duration}s`}
					values={get_fade_in_values()}
					keyTimes={get_keyTimes([5, 6])}
					repeatCount="indefinite"
				/>
			</rect>
		</Board>

		<!-- Existing Piece -->
		<Piece piece={end_hole_2x3_big} scale="1 -1" />

		<!-- New Piece -->
		<Piece piece={middle_hole_2x3_big}>
			<animateTransform
				attributeName="transform"
				dur={`${duration}s`}
				type="translate"
				values={get_move_values()}
				keyTimes={get_keyTimes([1, 2])}
				calcMode="spline"
				keySplines={move_spline}
				repeatCount="indefinite"
			/>
		</Piece>

		<!-- Piece Highlight -->
		<rect
			x={(padding + stroke_width / 2) * draw_scale}
			y={(padding + stroke_width / 2) * draw_scale}
			width={(1 - stroke_width) * draw_scale}
			height={(1 - stroke_width) * draw_scale}
		>
			<animateTransform
				attributeName="transform"
				dur={`${duration}s`}
				type="translate"
				values={get_move_values()}
				keyTimes={get_keyTimes([1, 2])}
				calcMode="spline"
				keySplines={move_spline}
				repeatCount="indefinite"
			/>
			<animate
				attributeName="fill"
				dur={`${duration}s`}
				values={get_fade_out_values()}
				keyTimes={get_keyTimes([3, 4])}
				repeatCount="indefinite"
			/>
		</rect>
	</svg>
</div>

<style>
	div.content {
		display: flex;
		margin: auto;
		align-items: center;
		justify-content: center;
	}
</style>
