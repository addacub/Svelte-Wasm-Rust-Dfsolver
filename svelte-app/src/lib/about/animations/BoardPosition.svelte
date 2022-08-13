<script lang="ts">
	import Board from '$lib/about/Board.svelte';
	import { board } from '$lib/about/board';

	// Input properties
	const width: number = board.width;
	const height: number = board.height;
	const stroke_width: number = board.stroke_width;
	const padding: number = board.padding;
	const draw_scale: number = board.draw_scale;
	const viewBox_width: number = width + padding * 2;
	const viewBox_height: number = height + padding * 2;

	const duration: number = 20; // duration of animation in seconds

	const empty_yellow: string = 'rgba(253, 252, 151, 0)';
	const yellow: string = 'rgba(253, 252, 151, 1)';
	const blue: string = 'rgba(71, 157, 255, 1)';
	const empty_blue: string = 'rgba(71, 157, 255, 0)';

	const elements: number = 12 + 31; // months + days
	const states: number = 6; // empty; empty; yellow; yellow; blue; blue; none
	const offset: number = 3;
	const delay: number = 0.05;
	const increment: number = (1 - delay) / (elements * offset + states);

	function get_keyTimes(index: number): string {
		let key_times: string[] = ['0'];

		if (index <= 12) {
			index -= Math.trunc(index / 7);
		} else {
			// index > 12
			index -= 2;
		}

		for (let i = 0; i < states - 2; i++) {
			key_times.push((delay + (index * offset + i) * increment).toString());
		}

		key_times.push((delay + (elements * offset + states - 1) * increment).toString());
		key_times.push('1');

		return key_times.join(';');
	}

	function get_yellow_values(): string {
		let values: string[] = new Array();

		values.push(empty_yellow);
		values.push(empty_yellow);
		values.push(yellow);
		values.push(yellow);
		values.push(empty_yellow);
		values.push(empty_yellow);
		values.push(empty_yellow);

		return values.join(';');
	}

	function get_blue_values(): string {
		let values: string[] = new Array();

		values.push(empty_blue);
		values.push(empty_blue);
		values.push(empty_blue);
		values.push(empty_blue);
		values.push(blue);
		values.push(blue);
		values.push(empty_blue);

		return values.join(';');
	}
</script>

<div class="content">
	<svg width={viewBox_width * draw_scale} height={viewBox_height * draw_scale}>
		<!-- Board -->
		<Board {board}>
			<!-- Highlights -->
			{#each new Array(height) as _, row}
				{#each new Array(width) as _, col}
					{#if ![6, 13, 45, 46, 47, 48].includes(width * row + col)}
						<rect
							x={(padding + stroke_width / 2 + col) * draw_scale}
							y={(padding + stroke_width / 2 + row) * draw_scale}
							width={(1 - stroke_width) * draw_scale}
							height={(1 - stroke_width) * draw_scale}
						>
							<animate
								dur={`${duration}s`}
								repeatCount="indefinite"
								attributeName="fill"
								values={get_yellow_values()}
								keyTimes={get_keyTimes(row * width + col)}
							/>
						</rect>
					{/if}
				{/each}
			{/each}
		</Board>

		<!-- Highlights -->
		{#each new Array(height) as _, row}
			{#each new Array(width) as _, col}
				{#if ![6, 13, 45, 46, 47, 48].includes(width * row + col)}
					<rect
						x={(padding + stroke_width / 2 + col) * draw_scale}
						y={(padding + stroke_width / 2 + row) * draw_scale}
						width={(1 - stroke_width) * draw_scale}
						height={(1 - stroke_width) * draw_scale}
					>
						<animate
							dur={`${duration}s`}
							repeatCount="indefinite"
							attributeName="fill"
							values={get_blue_values()}
							keyTimes={get_keyTimes(row * width + col)}
						/>
					</rect>
				{/if}
			{/each}
		{/each}
	</svg>
</div>

<style>
	div.content {
		display: flex;
		margin: auto;
	}
</style>
