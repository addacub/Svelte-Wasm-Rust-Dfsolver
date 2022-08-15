<script lang="ts">
	import katex from 'katex';
	export let math: string;
	export let displayMode: boolean;

	const options = {
		displayMode: displayMode,
		throwOnError: false
	};

	$: katexString = katex.renderToString(math, options);
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.css"
		integrity="sha384-Xi8rHCmBmhbuyyhbI88391ZKP2dmfnOl4rT9ZfRI7mLTdk1wblIUnrIq35nqwEvC"
		crossorigin="anonymous"
	/>
</svelte:head>

{@html katexString}
