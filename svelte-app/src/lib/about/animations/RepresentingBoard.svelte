<script lang="ts">
	import { board_rep } from '$lib/about/board';
	import Katex from '$lib/about/Katex.svelte';
	import Board from '$lib/about/Board.svelte';
	import type { BoardInfo } from '../board';

	// Input properties
	const board: BoardInfo = board_rep;
	const width: number = board.width;
	const height: number = board.height;
	const padding: number = board.padding;
	const draw_scale: number = board.draw_scale;

	const duration: number = 15;
	const spacing: number = 1.5;
	const arrow_length: number = 3;
	const viewBox_width: number = width + spacing * 2 + arrow_length + padding * 2;
	const viewBox_height: number = height + padding * 2;

	const edge_elements: number[] = [6, 13, 45, 46, 47, 48];
	const date_elements: number[] = [1, 27];

	const date_fill: string = 'rgba(253, 252, 151, 0.5)';

	function get_matrix(): string {
		let matrix: string = '\\begin{bmatrix} ';
		let col_elements: string[] = [];
		let row_elements: string[] = [];

		for (let row: number = 0; row < height; row++) {
			for (let col: number = 0; col < width; col++) {
				col_elements.push(
					edge_elements.includes(width * row + col) || date_elements.includes(width * row + col)
						? '1'
						: '0'
				);
			}
			row_elements.push(col_elements.join(' & '));
			col_elements = [];
		}

		return matrix.concat(row_elements.join(' \\\\ '), ' \\end{bmatrix}');
	}
</script>

<div class="content">
	<svg width="90%" viewBox="0 0 {viewBox_width * draw_scale} {viewBox_height * draw_scale}">
		<!-- Initial Piece -->
		<Board {board} font_size="small" />

		<!-- Date to Solve for Highlights -->
		<rect
			x={(1 + padding + board.stroke_width / 2) * draw_scale}
			y={(padding + board.stroke_width / 2) * draw_scale}
			width={(1 - board.stroke_width) * draw_scale}
			height={(1 - board.stroke_width) * draw_scale}
			fill={date_fill}
			stroke="none"
		/>;
		<rect
			x={(6 + padding + board.stroke_width / 2) * draw_scale}
			y={(3 + padding + board.stroke_width / 2) * draw_scale}
			width={(1 - board.stroke_width) * draw_scale}
			height={(1 - board.stroke_width) * draw_scale}
			fill={date_fill}
			stroke="none"
		/>;

		<!-- Adding Opaque rectangles -->
		<g class="squares" style={`animation-duration: ${duration}s;`}>
			{#each new Array(height) as _, row}
				{#each new Array(width) as _, col}
					{#if !edge_elements.includes(width * row + col)}
						<rect
							x={(col + padding + board.stroke_width / 2) * draw_scale}
							y={(row + padding + board.stroke_width / 2) * draw_scale}
							width={(1 - board.stroke_width) * draw_scale}
							height={(1 - board.stroke_width) * draw_scale}
							stroke="none"
						/>;
					{/if}
				{/each}
			{/each}
		</g>

		<!-- Adding numbers -->
		<g class="numbers" style={`animation-duration: ${duration}s;`}>
			{#each new Array(height) as _, row}
				{#each new Array(width) as _, col}
					<text
						class="numbers"
						x={(padding + board.stroke_width / 2 + 1 / 2 + col) * draw_scale}
						y={(padding + board.stroke_width / 2 + 1 / 2 + row) * draw_scale}
					>
						{edge_elements.includes(width * row + col) || date_elements.includes(width * row + col)
							? 1
							: 0}
					</text>
				{/each}
			{/each}
		</g>

		<!-- arrowhead marker definition  -->
		<defs>
			<marker
				id="arrow-tip-board"
				class="arrow-tip"
				viewBox="0 0 10 10"
				style={`animation-duration: ${duration}s;`}
				refX="0.05"
				refY="5"
				markerWidth="3"
				markerHeight="3"
				orient="auto-start-reverse"
				fill="none"
				stroke="none"
			>
				<path d="M 0 0 L 10 5 L 0 10 z" />
			</marker>
		</defs>

		<polyline
			class="arrow-body"
			style={`animation-duration: ${duration}s;`}
			points="{(width + spacing) * draw_scale},{(height / 2) * draw_scale} {(width +
				spacing +
				arrow_length) *
				draw_scale},{(height / 2) * draw_scale}"
			fill="none"
			marker-end="url(#arrow-tip-board)"
			stroke-width="12"
		/>
	</svg>
	<div class="katex_container" style={`animation-duration: ${duration}s;`}>
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

	svg > g > text {
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

	.squares {
		animation-name: square-fadein;
		animation-iteration-count: infinite;
	}

	@keyframes square-fadein {
		0% {
			fill: rgba(0, 0, 0, 0);
		}
		20% {
			fill: rgba(0, 0, 0, 0);
		}

		30% {
			fill: rgba(0, 0, 0, 0.5);
		}
		90% {
			fill: rgba(0, 0, 0, 0.5);
		}
		100% {
			fill: rgba(0, 0, 0, 0);
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
		font-size: x-large;
		position: relative;
		animation-name: matrix-fadein;
		animation-iteration-count: infinite;
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

	@media (max-width: 760px) {
		div.katex_container {
			font-size: large;
		}
	}

	@media (max-width: 480px) {
		div.katex_container {
			font-size: x-small;
		}
	}
</style>
