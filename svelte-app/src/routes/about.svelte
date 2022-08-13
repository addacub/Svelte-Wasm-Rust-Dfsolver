<script context="module">
	import { browser, dev } from '$app/env';
	export const hydrate = dev;
	export const router = browser;
	export const prerender = true;
</script>

<script lang="ts">
	import Katex from '$lib/about/Katex.svelte';
	import Piece from '$lib/about/Piece.svelte';
	import { table_pieces } from '$lib/about/piece';
	import BoardPosition from '$lib/about/animations/BoardPosition.svelte';
	import PlacePiece from '$lib/about/animations/PlacePiece.svelte';
	import ValidMove from '$lib/about/animations/ValidMove.svelte';
	import TranslatePiece from '$lib/about/animations/TranslatePiece.svelte';
	import PieceOrientations from '$lib/about/animations/PieceOrientations.svelte';
	import graphStream_webm from '$lib/assets/graphstreamMovie.webm';
	import tree_Diagram from '$lib/assets/TreeDiagram.png';
	import UniqueOrientations from '$lib/about/animations/UniqueOrientations.svelte';
	import RotationalSymmetry from '$lib/about/animations/RotationalSymmetry.svelte';
	import ReflectionSymmetry from '$lib/about/animations/ReflectionSymmetry.svelte';
	import RepresentingPiece from '$lib/about/animations/RepresentingPiece.svelte';
	import RepresentingBoard from '$lib/about/animations/RepresentingBoard.svelte';
	import { TableHeadings } from '$lib/about/store';

	let figure_count: number = 0;
	let table_count: number = 0;

	function get_figure_count(): number {
		figure_count++;
		return figure_count;
	}

	function get_table_count(): number {
		table_count++;
		return table_count;
	}
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
		/> is the number of columns. The elements of the matrix are used to represent the shape of a puzzle
		piece and the board with 0's representing holes and 1's representing filled spaces. Below is an example
		of how a puzzle piece is represented as a matrix:
	</p>

	<div class="highlight_container">
		<RepresentingPiece />
	</div>
	<p class="figure">
		<i>Figure {get_figure_count()}. How a puzzle piece is represented as a matrix in the code.</i>
	</p>

	<p>
		When representing the board, the board is squared off, with the boundary pieces filled with 1's.
		The target date is also filled with 1's and not a 0's to indicate that a piece cannot be placed
		there.
	</p>

	<div class="highlight_container">
		<RepresentingBoard />
	</div>
	<p class="figure">
		<i>Figure {get_figure_count()}. How the board is represented as a matrix in the code.</i>
	</p>

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

	<p>
		<i>Table {get_table_count()}. The puzzle pieces which can be placed.</i>
	</p>
	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>{TableHeadings.piece}</th>
				{#each table_pieces as piece}
					<td>
						<svg
							width={(piece.width + piece.padding) * piece.draw_scale}
							height={(piece.height + piece.padding) * piece.draw_scale}
						>
							<Piece {piece} />
						</svg>
					</td>
				{/each}
			</tr>
		</table>
	</div>

	<p>
		There are 8 puzzle pieces to choose from and 8 pieces need to be placed. The number of
		permutations is: <Katex math={'8! = 40320'} displayMode={true} />
		<b>BUT</b> each piece represents more than 1 piece due to rotations and flips.
	</p>

	<h3>Full Solution</h3>

	<p>
		By taking into the account rotations and flips, each puzzle piece is actually 1 of 8 unique
		pieces which can be placed. For example, the 2&#215;3 zig-zag has 8 unique orientations:
	</p>

	<div class="highlight_container"><UniqueOrientations /></div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. The 8 unique orientations of the the 2&#215;3 zig-zag piece. The
			top row shows the 4 unique rotational orientations of the piece (rotated by 90&#176;). The
			second row shows the 4 unique rotational orientations of the piece when flipped.</i
		>
	</p>

	<p>Now, finding a solution can be described as traversing through a tree structure:</p>

	<div class="highlight_container">
		<img src={tree_Diagram} alt="A diagram of a tree structure what explains terminology" />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. A diagram of a tree structure with labelled parts: root node,
			edge, parent node, child node, sub-tree, leaf node, siblings and levels</i
		>
	</p>

	<ul>
		<li>
			<b>Node:</b> contains data and may link to other nodes. The basic unit of a data structure. In
			this particular use case, a node represents the unique orientation of a puzzle piece that is placed.
		</li>
		<li>
			<b>Root node:</b> the node at the top of the tree which has no parent. There is only one root per
			tree and one path from the root node to any other node in the tree structure. The root node corresponds
			to 0 pieces (i.e. an empty board).
		</li>
		<li>
			<b>Child node:</b> any node which extends from another node.
		</li>
		<li>
			<b>Parent node:</b> the node which a child node extends from.
		</li>
		<li>
			<b>Leaf node:</b> a node with no children.
		</li>
		<li>
			<b>Siblings:</b> nodes which extend from the same parent node.
		</li>
		<li>
			<b>Edge:</b> the connection between nodes.
		</li>
		<li>
			<b>Degree:</b> for a given node is its number of children (corresponds to the number of unique
			piece orientations to choose from for the next turn).
		</li>
		<li>
			<b>Path:</b> the sequence of nodes between two given nodes in the tree structure.
		</li>
		<li>
			<b>Level: </b> for a given node is the number of edges along the unique path between it and the
			root node. The level number equals to how many pieces have been placed. Each level corresponds
			to a piece type. For example, the first level may correspond to placing the L-shape piece in one
			of its 8 unique orientations.
		</li>
		<li>
			<b>Sub-tree:</b> represents the descendants of a node.
		</li>
		<li>
			<b>Traversal:</b> the process of visiting each node in the tree exactly once.
		</li>
		<li>
			<b>Pruning:</b> Removing a whole section of a tree.
		</li>
	</ul>

	<p>
		The total number of permutations is calculated as follows:
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
		<li><b>d</b>: is an array of the degrees of the nodes at level <i>i</i>.</li>
	</ul>

	<p>The table below summaries the number of rotations and flips for each piece.</p>

	<p>
		<i
			>Table {get_table_count()}. Summarises how many unique rotational positions, flipped
			(reflection) positions, and total unique orientations a piece has.</i
		>
	</p>
	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>{TableHeadings.piece}</th>
				{#each table_pieces as piece}
					<td>
						<svg
							width={(piece.width + piece.padding) * piece.draw_scale}
							height={(piece.height + piece.padding) * piece.draw_scale}
						>
							<Piece {piece} />
						</svg>
					</td>
				{/each}
			</tr>
			<tr>
				<th>{TableHeadings.rotations}</th>
				{#each new Array(8) as i}
					<td>4</td>
				{/each}
			</tr>
			<tr>
				<th>{TableHeadings.flips}</th>
				{#each new Array(8) as i}
					<td>2</td>
				{/each}
			</tr>
			<tr>
				<th>{@html TableHeadings.unique_orient_math}</th>
				{#each new Array(8) as i}
					<td>8</td>
				{/each}
			</tr>
		</table>
	</div>

	<p>
		Therefore, the total number of permutations using equations (1) and (2) is:
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
		symmetry. Rotational symmetry is when a shape can be rotated by <i>n</i> degrees and appear unchanged:
	</p>

	<div class="highlight_container"><RotationalSymmetry /></div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. The rotational symmetry of the 2&#215;3 rectangular piece. It
			has a order of rotational symmetry of 2. This means when the piece is rotated by 180&#176; it
			appears unchanged.
		</i>
	</p>

	<p>Reflection (or mirror) symmetry is when the mirror image of a shape appears unchanged:</p>

	<div class="highlight_container"><ReflectionSymmetry /></div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. The reflection symmetry of the 2&#215;3 rectangular piece. Its
			mirror image appears unchanged and can be overlaid on top of the original image.
		</i>
	</p>

	<p>
		Therefore, by taking into account the symmetry, the number of permutations that need to be
		tested is reduced. The table below summaries the updated number of rotations and flips for each
		piece
	</p>

	<p>
		<i
			>Table {get_table_count()}. Summarises how many unique rotational positions, flipped
			(reflection) positions, and total unique orientations a piece has when taking into account
			rotational and reflection (mirror) symmetry.</i
		>
	</p>
	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>{TableHeadings.piece}</th>
				{#each table_pieces as piece}
					<td>
						<svg
							width={(piece.width + piece.padding) * piece.draw_scale}
							height={(piece.height + piece.padding) * piece.draw_scale}
						>
							<Piece {piece} />
						</svg>
					</td>
				{/each}
			</tr>
			<tr>
				<th>{TableHeadings.rotations}</th>
				<td>2</td>
				{#each new Array(5) as i}
					<td>4</td>
				{/each}
				<td>2</td>
				<td>4</td>
			</tr>
			<tr>
				<th>{TableHeadings.flips}</th>
				<td>1</td>
				<td>1</td>
				{#each new Array(5) as i}
					<td>2</td>
				{/each}
				<td>1</td>
			</tr>
			<tr>
				<th>{@html TableHeadings.unique_orient_math}</th>
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
		Therefore, the total number of permutations using equations (1) and (2) is:
		<Katex math={'\\boldsymbol{d} = (2, 4, 8, 8, 8, 8, 4, 4)'} displayMode={true} />
		<Katex
			math={'\\begin{equation} \\notag \\begin{split} P(8, \\boldsymbol{d}) &= 8! \\times \\prod_{i=0}^7\\boldsymbol{d}_i \\\\ &= 8! \\times (2 \\times 4 \\times 8 \\times 8 \\times 8 \\times 8 \\times 4 \\times 4) \\\\ &= 2.113929216 \\times 10^{10} \\end{split} \\end{equation}'}
			displayMode={true}
		/>
	</p>

	<h2>Where a Piece is Placed</h2>

	<p>
		The first available space (board position) is found by scanning across the board left to right
		and top to bottom, like reading a sentence:
	</p>

	<div class="highlight_container">
		<BoardPosition />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. The next available board position is found by scanning across
			the board left to right and top to bottom, like reading a sentence.
		</i>
	</p>

	<p>
		A puzzle piece is placed on the first available square on the board. When placing a piece, the
		first "square" of the piece is placed on top of the first available square on the board:
	</p>

	<div class="highlight_container">
		<PlacePiece />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. The first "square" of the piece is placed on top of the first
			available square on the board as determined in Figure {figure_count - 1}.
		</i>
	</p>

	<p>Note, a puzzle piece is only placed if it is a valid position.</p>

	<h2>Definition of Valid</h2>

	<p>
		Before placing a piece in one if its unique orientations, the following questions are asked to
		determine if the piece will be placed in a valid position:
	</p>
	<ul>
		<li>Will the puzzle piece go outside the bounds of the puzzle board?</li>
		<li>
			Will the piece overlap with any pieces that have been placed or the target date (target date
			is a filled square)?
		</li>
		<li>Will placing the piece leave any unreachable holes?</li>
	</ul>
	<p>
		A hole (an element of the board matrix with a value of 0) is any board position which hasn't
		been covered with a piece and is not the target date. If a hole is too small, no piece can be
		placed there without overlapping with other pieces that have been placed. I.e it is unreachable.
	</p>

	<p>
		<b>
			If the answer to any of the above questions is yes, the position is not valid and the piece in
			its current unique orientation cannot be placed.
		</b>
	</p>

	<p>
		For example, assuming today's date is the 1st of September, if the purple piece is to be placed
		onto the next available board position, the May square becomes the next available board
		position. However, no piece can be placed on the May square without overlapping with the already
		existing piece. It is <b>unreachable</b>. Hence, this is not a valid position.
	</p>

	<div class="highlight_container">
		<ValidMove />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. Placing the 2&#215;3 middle hole piece at the next available
			board position results in a unreachable hole at the "May" square. Hence, placing that piece in
			its current unique orientation is not valid.
		</i>
	</p>

	<h2>How a Piece is Placed</h2>

	<p>
		If the piece's unique orientation cannot be placed at the current available board position, the
		piece's orientation is changed or a new piece may be attempted to be placed instead. The process
		for exhausting all of a piece's orientations are governed by the rules below which start from
		the top each time a piece's orientation is changed:
	</p>
	<ul>
		<li>If the piece has not exhausted its rotations, rotate the piece by 90&#176.</li>
		<li>
			Else If the piece has exhausted its rotations, flip the piece if it can be flipped and has not
			already been flipped.
		</li>
		<li>Else move on to next available puzzle piece.</li>
	</ul>

	<p>
		Note, the number of rotations is for each side of a piece if it is flippable. I.e. each side has <i
			>n</i
		> unique rotational orientations. Refer to the tables for how many rotations and flips each piece
		can undergo. The below animation shows how a piece's orientation is exhausted. The steps consist
		of:
	</p>

	<ul>
		<li>Rotating the piece 3 times to obtain four unique rotational orientations.</li>
		<li>Flipping the piece.</li>
		<li>Rotating the piece 3 times to obtain four new unique rotational orientations.</li>
		<li>Flipping the piece to end up back at its starting orientation.</li>
	</ul>

	<div class="highlight_container">
		<PieceOrientations />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. How the 8 unique orientations of the the 2&#215;3 zig-zag piece
			are exhausted. First the piece is rotated through all of its unique rotational positions.
			Then, flipped (if it can be flipped). And again, rotated through all of its unique rotational
			positions.
		</i>
	</p>

	<p>
		However, with only these 3 steps, not all valid solutions will be found. We also need to account
		for puzzle piece orientations with holes in the top row of a puzzle piece. The rules for
		exhausting all of a piece's orientations which start from the top each time a piece's
		orientation is changed are now:
	</p>
	<ul>
		<li>
			If the first "square" of the piece is a hole, move the piece to the left by
			<i>n</i> squares until the next available board position aligns with the piece's first filled square
			on the top row.
		</li>
		<li>Else if the piece has not exhausted its rotations, rotate the piece by 90&#176.</li>
		<li>
			Else if the piece has exhausted its rotations, flip the piece if it can be flipped and has not
			already been flipped.
		</li>
		<li>Else move on to next available puzzle piece.</li>
	</ul>

	<p>
		The animation below shows how moving pieces to the left, if there are holes in the top row,
		allows pieces to be placed on the board in valid positions which were previously unreachable
		based on the old rules.
	</p>

	<div class="highlight_container">
		<TranslatePiece />
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. How the 8 unique orientations of the the 2&#215;3 zig-zag piece
			are exhausted. First the piece is rotated through all of its unique rotational positions.
			Then, flipped (if it can be flipped). And again, rotated through all of its unique rotational
			positions.
		</i>
	</p>

	<h2>Writing the Algorithm</h2>

	<p>
		The solver algorithm is a depth-first tree traversal with branch pruning. The video below
		(Figure {figure_count + 1}) is a visualisation of how the solver traverses through the tree of
		the possible permutations. The tree only contains the first 4 levels (i.e. the first 4 pieces
		being placed) with the order of piece shape fixed. The root node, L0, corresponds to an empty
		board (i.e. no pieces have been placed). The node levels and the pieces they represent are
		summarised in the table below:
	</p>

	<p>
		<i
			>Table {get_table_count()}. The puzzle pieces which can be placed, the level of the tree
			structure which they correspond to, and how many unique orientations they have.</i
		>
	</p>
	<div class="highlight_container">
		<table>
			<colgroup>
				<col span="1" style="width: 25%" />
			</colgroup>
			<tr>
				<th>{TableHeadings.level}</th>
				{#each table_pieces as _, i}
					<td>L{i + 1}</td>
				{/each}
			</tr>
			<tr>
				<th>{TableHeadings.piece}</th>
				{#each table_pieces as piece, index}
					<td>
						<svg
							width={(piece.width + piece.padding) * piece.draw_scale}
							height={(piece.height + piece.padding) * piece.draw_scale}
						>
							<Piece {piece} />
						</svg>
					</td>
				{/each}
			</tr>
			<tr>
				<th>{@html TableHeadings.unique_orient}</th>
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

	<p>Traversing through the tree is as follows:</p>

	<ul>
		<li>
			Starting with an empty board (i.e. the root node) try to place a piece at the next available
			square on the board (i.e. visit a node at L1, which is a child of the root node).
		</li>
		<li>
			If placing the piece is valid, another piece is attempted to be placed at the next available
			square on the board (i.e. visit a node at L2 which is a child node of the last visited L1
			node).
		</li>
		<li>
			If placing the piece is not valid, remove that piece and try another piece (i.e. the visited
			L2 node can be pruned and all its child nodes do not need to be visited. Visit another L2 node
			that is a child to the last visited L1 node).
		</li>
	</ul>

	<p>These steps are repeated until the whole tree has been traversed.</p>

	<p>Note the following:</p>
	<ul>
		<li>
			Orange nodes are nodes which are currently being visited (i.e. testing if placing that piece's
			unique orientation on the next available square on the board is valid).
		</li>
		<li>Red nodes are nodes which have already been visited (i.e. tested if valid).</li>
		<li>Grey nodes are nodes which been pruned.</li>
		<li>
			Branch pruning is arbitrary and is only for visualisation purposes. It does not correspond to
			an actual solution.
		</li>
	</ul>

	<p>
		The colour of the nodes which have not been visited correspond to the colour of the pieces in
		the table above.
	</p>

	<div class="highlight_container">
		<video width="100%" height="100%" autoplay muted controls loop src={graphStream_webm}>
			A video of a tree graph undergoing a depth first algorithm. As each node is visited, it turns
			red. If it is a invalid permutation, it is pruned, and all of its child nodes turn a
			translucent grey.
		</video>
	</div>
	<p class="figure">
		<i
			>Figure {get_figure_count()}. A video of tree traversal using a depth first algorithm. As each
			node is visited, it turns red. If it is a invalid permutation, it is pruned, and all of its
			children nodes turn a translucent grey.
		</i>
	</p>

	<p>
		The full tree for the fixed order of pieces as shown in the <i>Table {table_count}</i> has an additional
		4 levels (L5 - L8). And 8! (8 factorial) of these trees with the same the root node are needed to
		account for the permutations of the order of the pieces placed (i.e. when changing the order at which
		pieces are placed). Refer to Equation 1 and Equation 2.
	</p>
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
		font-size: smaller;
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

	ul {
		list-style-type: square;
	}

	li {
		padding: 0.5rem 0;
	}

	img {
		width: 100%;
	}

	p.figure {
		padding-bottom: 1rem;
	}
</style>
