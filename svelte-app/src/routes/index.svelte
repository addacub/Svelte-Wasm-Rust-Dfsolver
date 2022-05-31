<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Calendar from '$lib/Calendar.svelte';
	import Menu from '$lib/Menu.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section class="row">
	<div class="column left">
		<Menu />
	</div>
	<div class="column right">
		<Calendar />
	</div>
</section>

<style>
	/* section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 2;
	} */

	.row {
		display: flex;
	}

	.column {
		float: left;
	}

	.left {
		width: 25%;
		margin-right: 10rem;
	}

	.right {
		width: 75%;
	}
</style>
