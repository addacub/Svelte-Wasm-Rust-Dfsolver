<script context="module">
	import { browser, dev } from '$app/env';

	// we don't need any JS on this page, though we'll load
	// it in dev so that we get hot module replacement...
	export const hydrate = dev;

	// ...but if the client-side router is already loaded
	// (i.e. we came here from elsewhere in the app), use it
	export const router = browser;

	// since there's no dynamic data here, we can prerender
	// it so that it gets served as a static asset in prod
	export const prerender = true;
</script>

<script lang="ts">
	import Katex from '$lib/about/Katex.svelte';
	import Piece from '$lib/about/TablePiece.svelte';
	import { pieces } from '$lib/about/piece';
	import AnimatedPiece from '$lib/about/AnimatedPiece.svelte';
</script>

<svelte:head>
	<title>About</title>
	<meta name="description" content="About this app" />
</svelte:head>

<div class="content">
	<h1>Welcome!</h1>

	<h2>About This App</h2>

	<p>
		This is a solver for DragonFjord's <a
			href="https://www.dragonfjord.com/product/a-puzzle-a-day/"
			target="_blank">A-Puzzle-A-Day</a
		>. The solver is a <a href="https://kit.svelte.dev" target="_blank">SvelteKit</a> application,
		with the main logic written in <a href="https://www.rust-lang.org/" target="_blank">Rust</a> and
		compiled into
		<a href="https://webassembly.org/" target="_blank">WASM</a>.
	</p>

	<h2>Representing the Problem</h2>

	<p>
		The board and the puzzle pieces are represented within the code as <Katex
			math={'M \\times N'}
			displayMode={false}
		/> matrices where <Katex math={'M'} displayMode={false} /> is the number of rows and <Katex
			math={'N'}
			displayMode={false}
		/> is the number of columns. The elements of the matrix are used to represent the shape of the puzzle
		piece with 0's representing holes and 1's representing filled spaces. Below is an example of how
		a puzzle piece is represented as a matrix:
	</p>

	<div class="highlight_container">
		<AnimatedPiece />
	</div>

	<h2>The Solution Space</h2>

	<p>
		A solution to the A-Puzzle-A-Day is when the puzzle pieces are placed on the board such that
		only today's day and month are visible. To get to a solution, numerous permutations need to be
		tested. The number of permutations of <i>n</i> distinct options is <i>n</i> <b>factorial</b>.
	</p>

	<p>
		Note, in mathematics the <b>factorial</b> of a non-negative integer <i>n</i>, denoted by
		<i>n</i>!, is the product of all positive integers less than or equal to <i>n</i>. For example:
		<Katex math={'5! = 5 \\times 4 \\times 3 \\times 2 \\times 1 = 120'} displayMode={false} />.
	</p>

	<h3>Naïve Solution</h3>

	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>Pieces</th>
				{#each pieces as piece}
					<td
						><Piece
							width={piece.get_width()}
							height={piece.get_height()}
							svg_path={piece.get_path()}
							hsl={piece.get_fill()}
						/></td
					>
				{/each}
			</tr>
		</table>
	</div>

	<p>
		There are 8 puzzle pieces to choose from and 8 pieces need to be placed. The number of
		permutations is: <Katex math={'8! = 40,320'} displayMode={true} />
		<b>BUT</b> each piece represents more than 1 piece due to rotations and flips.
	</p>

	<h3>Full Solution</h3>

	<p>
		By taking into the account rotations and flips, each puzzle piece is actually 1 of 8 unique
		pieces which can be placed. Now, a permutation can be described as traversing through a tree
		like structure where:
	</p>
	<ul>
		<li>A <b>node</b> corresponds to the piece that is placed.</li>
		<li>
			The <b>degree</b> for a given node is its number of children (corresponds to the number of pieces
			to choose from).
		</li>
		<li>
			The <b>level</b> of a node is the number of edges along the unique path between it and the root
			node. The level number equals to how many pieces have been placed. Each level corresponds to a
			piece type. For example, the first level may correspond to placing the L-shape piece in one of
			its 8 unique orientations.
		</li>
	</ul>
	<p>
		Therefore, the total number of permutations is calculated as follows:
		<Katex
			math={'\\begin{equation}\\boldsymbol{d} = (d_0, d_1, d_2, ..., d_N)\\end{equation}'}
			displayMode={true}
		/>
		<Katex
			math={'\\begin{equation} P(L, \\boldsymbol{d}) = L! \\times \\prod_{i=0}^N\\boldsymbol{d}_i \\end{equation}'}
			displayMode={true}
		/>

		where:
	</p>
	<ul>
		<li><i>L</i>: is the total number of levels (i.e. how many pieces need to be placed).</li>
		<li><b>d</b>: is an array of the degrees of each node at level <i>i</i>.</li>
	</ul>

	<p>The table below summaries the number of rotations and flips for each piece.</p>

	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>Pieces</th>
				{#each pieces as piece}
					<td
						><Piece
							width={piece.get_width()}
							height={piece.get_height()}
							svg_path={piece.get_path()}
							hsl={piece.get_fill()}
						/></td
					>
				{/each}
			</tr>
			<tr>
				<th>Rotations</th>
				{#each new Array(8) as i}
					<td>4</td>
				{/each}
			</tr>
			<tr>
				<th>Flips</th>
				{#each new Array(8) as i}
					<td>2</td>
				{/each}
			</tr>
			<tr>
				<th>Total Unique Pieces</th>
				{#each new Array(8) as i}
					<td>8</td>
				{/each}
			</tr>
		</table>
	</div>

	<p>
		Therefore, the total number of permuations using equations (1) and (2) is:
		<Katex math={'\\boldsymbol{d} = (8, 8, 8, 8, 8, 8, 8, 8)'} displayMode={true} />
		<Katex
			math={'\\begin{equation} \\notag \\begin{split} P(8, \\boldsymbol{d}) &= 8! \\times \\prod_{i=0}^7\\boldsymbol{d}_i \\\\ &= 8! \\times (8 \\times 8 \\times 8 \\times 8 \\times 8 \\times 8 \\times 8 \\times 8) \\\\ &= 6.7645734912 \\times 10^{11} \\end{split} \\end{equation}'}
			displayMode={true}
		/>
		For comparison, there are estimated to be <Katex
			math={'2 \\times 10^{11}'}
			displayMode={false}
		/> to <Katex math={'4 \\times 10^{11}'} displayMode={false} />
		<a href="https://en.wikipedia.org/wiki/Milky_Way" target="_blank">stars in the milky way</a>.
	</p>

	<h3>Smart Solution</h3>
	<p>
		Not all of the pieces are unique, as some pieces have rotational symmetry and reflection
		symmetry. Therefore, by taking into account the symmetry the number of permuations that need to
		be tested is reduced. The table below summaries the updated number of rotations and flips for
		each piece
	</p>

	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>Pieces</th>
				{#each pieces as piece}
					<td
						><Piece
							width={piece.get_width()}
							height={piece.get_height()}
							svg_path={piece.get_path()}
							hsl={piece.get_fill()}
						/></td
					>
				{/each}
			</tr>
			<tr>
				<th>Rotations</th>
				<td>2</td>
				{#each new Array(5) as i}
					<td>4</td>
				{/each}
				<td>2</td>
				<td>4</td>
			</tr>
			<tr>
				<th>Flips</th>
				<td>1</td>
				<td>1</td>
				{#each new Array(5) as i}
					<td>2</td>
				{/each}
				<td>1</td>
			</tr>
			<tr>
				<th>Total Unique Pieces</th>
				<td>2</td>
				<td>4</td>
				{#each new Array(4) as i}
					<td>8</td>
				{/each}
				<td>4</td>
				<td>4</td>
			</tr>
		</table>
	</div>

	<p>
		Therefore, the total number of permuations using equations (1) and (2) is:
		<Katex math={'\\boldsymbol{d} = (2, 4, 8, 8, 8, 8, 4, 4)'} displayMode={true} />
		<Katex
			math={'\\begin{equation} \\notag \\begin{split} P(8, \\boldsymbol{d}) &= 8! \\times \\prod_{i=0}^7\\boldsymbol{d}_i \\\\ &= 8! \\times (2 \\times 4 \\times 8 \\times 8 \\times 8 \\times 8 \\times 4 \\times 4) \\\\ &= 2.113929216 \\times 10^{10} \\end{split} \\end{equation}'}
			displayMode={true}
		/>
	</p>

	<h2>Where a Piece is Placed</h2>

	<p>
		A puzzle piece is placed on the board at the first available space. The next available space
		(board position) is found by scanning across the board left to right and top to bottom, like
		reading a sentence. The puzzle piece is only placed if it is a valid position.
	</p>

	<h2>Definition of Valid</h2>

	<h2>How a Piece is Placed</h2>
</div>

<style>
	div.content {
		width: 100%;
		max-width: 52rem;
		margin: var(--column-margin-top) auto 0 auto;
		overflow: visible;
	}

	@media screen and (max-width: 620px) and (orientation: portrait),
		screen and (max-height: 760px) and (orientation: landscape) {
		.content {
			margin: 0 auto 0 auto;
		}

		h1 {
			font-size: 2.4rem;
		}

		h2 {
			font-size: 1.5rem;
		}
	}

	table {
		border-collapse: collapse;
		width: 100%;
		align-content: center;
		margin: auto;
	}

	tr:nth-child(even) {
		background-color: #f2f2f2;
	}

	tr:nth-child(odd) {
		background-color: lightgray;
	}

	tr:hover {
		background-color: #ddd;
	}

	th {
		padding: 8px;
		text-align: left;
		background-color: #2d5358;
		border: 1px solid #ddd;
	}

	td {
		border: 1px solid #ddd;
		padding: 8px;
		text-align: center;
	}

	td {
		color: black;
	}

	div.highlight_container {
		font-size: medium;
		font-family: var(--font-mono);
		background-color: rgba(255, 255, 255, 0.45);
		border-radius: 3px;
		box-shadow: 2px 2px 6px rgb(255 255 255 / 25%);
		padding: 0.5em;
		overflow: auto;
		color: var(--text-color);
		/* Centre vertically and horizontally */
		display: flex;
		align-items: center;
	}
</style>
