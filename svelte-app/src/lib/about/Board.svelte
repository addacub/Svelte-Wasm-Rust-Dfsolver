<script lang="ts">
	import { getStrokeLightness } from '$lib/utils/utils';
	import type { BoardInfo } from '$lib/about/board';

	export let board: BoardInfo;
	export let font_size: string = 'large';

	// Input properties
	const width: number = board.width;
	const height: number = board.height;
	const svg_path: string = board.path;
	const stroke_width: number = board.stroke_width;
	const padding: number = board.padding;
	const draw_scale: number = board.draw_scale;
	const hsl: [number, number, number] = board.fill;

	// Styling variables
	const fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	const stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;

	let months: readonly String[] = [
		'Jan',
		'Feb',
		'Mar',
		'Apr',
		'May',
		'Jun',
		'Jul',
		'Aug',
		'Sep',
		'Oct',
		'Nov',
		'Dec'
	];

	const month_rows: number = 2;
	const month_cols: number = 6;
	const day_rows: number = 5;
	const day_cols: number = 7;
</script>

<svg
	width={(width + padding * 2) * draw_scale}
	height={(height + padding * 2) * draw_scale}
	display="inline-block"
>
	<!-- Drawing basic shape -->
	<path d={svg_path} {fill} />

	<!-- Drawing vertical lines -->
	{#each new Array(width + 1) as _, i}
		<line
			x1={(i + padding) * draw_scale}
			y1={((i === width ? 2 : 0) + padding - stroke_width / 2) * draw_scale}
			x2={(i + padding) * draw_scale}
			y2={((i >= 4 ? -1 : 0) + padding + height + stroke_width / 2) * draw_scale}
			{stroke}
			stroke-width={stroke_width * draw_scale}
		/>
	{/each}

	<!-- Drawing horizontal lines -->
	{#each new Array(height + 1) as _, i}
		<line
			x1={(padding - stroke_width / 2) * draw_scale}
			y1={(i + padding) * draw_scale}
			x2={((i <= 1 ? -1 : i === width ? -4 : 0) + padding + width + stroke_width / 2) * draw_scale}
			y2={(i + padding) * draw_scale}
			{stroke}
			stroke-width={stroke_width * draw_scale}
		/>
	{/each}

	<!-- Allow for highlights to be under text -->
	<slot />

	<!-- Adding Months -->
	{#each new Array(month_rows) as _, row}
		{#each new Array(month_cols) as _, col}
			<text
				class="months"
				x={(padding + 1 / 2 + col) * draw_scale}
				y={(padding + 1 / 2 + row) * draw_scale}
				font-size={font_size}
			>
				{months[month_cols * row + col]}
			</text>
		{/each}
	{/each}

	<!-- Adding Days -->
	{#each new Array(day_rows) as _, row}
		{#each new Array(day_cols) as _, col}
			<text
				class="months"
				x={(padding + 1 / 2 + col) * draw_scale}
				y={(padding + 1 / 2 + row + month_rows) * draw_scale}
				font-size={font_size}
			>
				{day_cols * row + col < 31 ? day_cols * row + col + 1 : ''}
			</text>
		{/each}
	{/each}
</svg>

<style>
	svg > text {
		fill: black;
		text-anchor: middle;
		dominant-baseline: middle;
	}
</style>
