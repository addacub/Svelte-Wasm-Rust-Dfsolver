<script lang="ts">
	import { middle_hole_2x3_rep } from '$lib/about/piece';
	import Katex from '$lib/about/Katex.svelte';
	import Piece from '$lib/about/Piece.svelte';
	import type { PieceInfo } from '../piece';

	// Input properties
	const piece: PieceInfo = middle_hole_2x3_rep;
	const width: number = piece.width;
	const height: number = piece.height;
	const padding: number = piece.padding;
	const draw_scale: number = piece.draw_scale;

	const duration: number = 10;
	const spacing: number = 1.5;
	const arrow_length: number = 3;
	const viewBox_width: number = width + spacing * 2 + arrow_length + padding * 2;
	const viewBox_height: number = height + padding * 2;

	function get_matrix(): string {
		return 'M\\bigg\\lbrace \\overbrace{\\begin{bmatrix} 1 & 0 & 1 \\\\ 1 & 1 & 1 \\end{bmatrix}}^{\\text{N}}';
	}
</script>

<div class="content">
	<svg width="90%" viewBox="0 0 {viewBox_width * draw_scale} {viewBox_height * draw_scale}">
		<!-- Initial Piece -->
		<Piece {piece} />

		<!-- Adding numbers -->
		{#each new Array(height) as _, row}
			{#each new Array(width) as _, col}
				<text
					class="numbers"
					style="animation-duration: {duration}s;"
					x={(padding + 1 / 2 + col) * draw_scale}
					y={(padding + 1 / 2 + row) * draw_scale}
				>
					{width * row + col === 1 ? 0 : 1}
				</text>
			{/each}
		{/each}

		<!-- arrowhead marker definition  -->
		<defs>
			<marker
				id="arrow-tip-piece"
				class="arrow-tip"
				style="animation-duration: {duration}s;"
				viewBox="0 0 10 10"
				refX="0.05"
				refY="5"
				markerWidth="3"
				markerHeight="3"
				orient="auto"
				stroke="none"
				fill="none"
			>
				<path d="M 0 0 L 10 5 L 0 10 z" />
			</marker>
		</defs>

		<polyline
			class="arrow-body"
			style="animation-duration: {duration}s;"
			points="{(width + spacing) * draw_scale},{(height / 2) * draw_scale} {(width +
				spacing +
				arrow_length) *
				draw_scale},{(height / 2) * draw_scale}"
			fill="none"
			stroke="none"
			marker-end="url(#arrow-tip-piece)"
			stroke-width="12"
		/>
	</svg>
	<div class="katex_container" style="animation-duration: {duration}s;">
		<Katex math={get_matrix()} displayMode={false} />
	</div>
</div>

<style>
	div.content {
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 2% 5%;
	}

	svg > text {
		text-anchor: middle;
		dominant-baseline: middle;
		font-size: x-large;
	}

	.numbers {
		animation-name: number-fadein;
		animation-iteration-count: infinite;
	}

	@keyframes number-fadein {
		0% {
			fill: rgba(255, 255, 255, 0);
		}
		20% {
			fill: rgba(255, 255, 255, 0);
		}

		30% {
			fill: rgba(255, 255, 255, 1);
		}
		90% {
			fill: rgba(255, 255, 255, 1);
		}
		100% {
			fill: rgba(255, 255, 255, 0);
		}
	}

	.arrow-tip {
		animation-name: arrow_tip_fadein;
		animation-iteration-count: infinite;
	}

	@keyframes arrow_tip_fadein {
		0% {
			fill: rgba(255, 255, 255, 0);
		}
		40% {
			fill: rgba(255, 255, 255, 0);
		}

		50% {
			fill: rgba(255, 255, 255, 1);
		}
		90% {
			fill: rgba(255, 255, 255, 1);
		}
		100% {
			fill: rgba(255, 255, 255, 0);
		}
	}

	.arrow-body {
		animation-name: arrow_body_fadein;
		animation-iteration-count: infinite;
	}

	@keyframes arrow_body_fadein {
		0% {
			stroke: rgba(255, 255, 255, 0);
		}
		40% {
			stroke: rgba(255, 255, 255, 0);
		}

		50% {
			stroke: rgba(255, 255, 255, 1);
		}
		90% {
			stroke: rgba(255, 255, 255, 1);
		}
		100% {
			stroke: rgba(255, 255, 255, 0);
		}
	}

	div.katex_container {
		margin: auto;
		font-size: xx-large;
		position: relative;
		animation-name: matrix-fadein;
		animation-iteration-count: infinite;
		padding-bottom: 3rem;
	}

	@keyframes matrix-fadein {
		0% {
			color: rgba(255, 255, 255, 0);
		}
		60% {
			color: rgba(255, 255, 255, 0);
		}

		70% {
			color: rgba(255, 255, 255, 1);
		}
		90% {
			color: rgba(255, 255, 255, 1);
		}
		100% {
			color: rgba(255, 255, 255, 0);
		}
	}

	@media (max-width: 480px) {
		div.katex_container {
			font-size: small;
			padding-bottom: 1.5rem;
		}
	}
</style>
