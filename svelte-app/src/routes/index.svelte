<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Calendar from '$lib/solver/Calendar.svelte';
	import Menu from '$lib/solver/Menu.svelte';
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
	.row {
		display: flex;
	}

	.column {
		float: left;
	}

	.left {
		width: 30%;
		margin-right: 2rem;
	}

	.right {
		width: 70%;
	}
</style>
