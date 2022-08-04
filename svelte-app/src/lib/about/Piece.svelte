<script lang="ts">
	import type { Piece } from '$lib/about/piece';
	import { getStrokeLightness } from '$lib/utils/utils';

	export let piece: Piece;

	// Input properties
	const width: number = piece.width;
	const height: number = piece.height;
	const svg_path: string = piece.path;
	const stroke_width: number = piece.stroke_width;
	const padding: number = piece.padding;
	const draw_scale: number = piece.draw_scale;
	const hsl: [number, number, number] = piece.fill;

	// Styling variables
	const fill: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${hsl[2]}%)`;
	const stroke: string = `hsl(${hsl[0]}, ${hsl[1]}%, ${getStrokeLightness(hsl[2])}%)`;
</script>

<g
	transform-origin={`${(width / 2 + padding) * draw_scale} ${(height / 2 + padding) * draw_scale}`}
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

	<!-- Custom animations go here -->
	<slot />
</g>
