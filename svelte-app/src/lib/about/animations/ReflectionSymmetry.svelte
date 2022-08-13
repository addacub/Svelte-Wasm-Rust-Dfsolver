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
</script>

<div class="content">
	<svg width={viewBox_width * piece.draw_scale} height={viewBox_height * piece.draw_scale}>
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
		</g>
	</svg>
</div>

<style>
	div.content {
		display: flex;
		margin: auto;
	}
</style>
