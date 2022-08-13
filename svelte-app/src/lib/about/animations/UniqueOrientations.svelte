<script lang="ts">
	import Piece from '../Piece.svelte';
	import { zig_zag_2x4_med } from '../piece';

	// Input properties
	const spacing: number = 1;
	const padding: number = zig_zag_2x4_med.padding;
	const width: number =
		(zig_zag_2x4_med.width + zig_zag_2x4_med.height) * 2 + spacing * 3 + padding * 2;
	const height: number = zig_zag_2x4_med.width * 2.5 + spacing + padding * 2;

	const draw_scale: number = zig_zag_2x4_med.draw_scale;

	let test: number = 1;
</script>

<div class="content">
	<svg width={width * draw_scale} height={height * draw_scale}>
		{#each new Array(4) as _, i}
			<g
				transform={`translate(
            ${(spacing * i + 3 * Math.ceil(i / 2) + 3 * Math.floor(i / 2)) * draw_scale} 
            ${(zig_zag_2x4_med.width / 2) * draw_scale}
            )`}
			>
				<Piece piece={zig_zag_2x4_med} rotate_deg={90 * i} scale="1 1" />
			</g>
			<g
				transform={`translate(
            ${(spacing * i + 3 * Math.ceil(i / 2) + 3 * Math.floor(i / 2)) * draw_scale} 
            ${((zig_zag_2x4_med.width * 3) / 2 + spacing) * draw_scale}
            )`}
			>
				<Piece piece={zig_zag_2x4_med} rotate_deg={90 * i} scale="1 -1" />
			</g>
		{/each}
	</svg>
</div>

<style>
	div.content {
		display: flex;
		margin: auto;
	}
</style>
