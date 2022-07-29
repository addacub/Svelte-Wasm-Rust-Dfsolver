<script lang="ts">
	import { getStrokeLightness } from '$lib/utils/utils';
	import { table_draw_scale, table_svg_padding } from './store';

	// Input properties
	export let width: number;
	export let height: number;
	export let svg_path: string;
	export let hsl: [number, number, number];

	let draw_scale: number = $table_draw_scale;
	let padding: number = $table_svg_padding;
	let stroke_width: number = 0.05;

	// Styling variables
	let fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	let stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;
</script>

<svg
	width={(width + padding * 2) * draw_scale}
	height={(height + padding * 2) * draw_scale}
	display="inline-block"
>
	<path d={svg_path} {fill} />
	<!-- Drawing vertical lines -->
	{#each new Array(width + 1) as _, i}
		<line
			x1={(i + padding) * draw_scale}
			y1={(padding - stroke_width / 2) * draw_scale}
			x2={(i + padding) * draw_scale}
			y2={(padding + height + stroke_width / 2) * draw_scale}
			{stroke}
			stroke-width={stroke_width * draw_scale}
		/>
	{/each}
	<!-- Drawing horizontal lines -->
	{#each new Array(height + 1) as _, i}
		<line
			x1={(padding - stroke_width / 2) * draw_scale}
			y1={(i + padding) * draw_scale}
			x2={(padding + width + stroke_width / 2) * draw_scale}
			y2={(i + padding) * draw_scale}
			{stroke}
			stroke-width={stroke_width * draw_scale}
		/>
	{/each}
</svg>
