<script lang="ts">
	let hints: number = 1;
	let solution: number;

	let solutions: Number[] = [1, 2, 3, 4, 5];
</script>

<div class="slide-container">
	<p>Select how many pieces to show:</p>
	<input bind:value={hints} type="range" min="0" max="8" step="1" />
	<output><b>{hints}</b></output>
</div>

<div class="spinner-container">
	<p>Select solution to show:</p>
	<input
		type="number"
		bind:value={solution}
		min="1"
		max={solutions ? solutions.length : 0}
		disabled={!solutions}
	/>
	{#if solutions}
		<p>{solutions.length} solutions were found.</p>
	{:else}
		<p>No solutions to select from.</p>
	{/if}
</div>

<div class="button-container">
	<button type="button"> Solve! </button>
</div>

<style>
	div {
		padding-top: 5rem;
	}

	input {
		display: block;
		margin: auto;
	}

	output {
		text-align: center;
		color: white;
		display: block;
		padding-top: 15px;
	}

	button {
		background-color: #c8adc0;
		width: 6em;
		height: 2em;
		font-size: xx-large;
		border-width: 5px;
	}

	button:hover {
		background-color: hsl(318, 20%, 50%);
	}
</style>
