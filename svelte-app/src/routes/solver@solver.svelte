<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Calendar from '$lib/solver/Calendar.svelte';
	import Menu from '$lib/solver/Menu.svelte';
	import { is_wasm_asset_fetched } from '$lib/solver/store';
	import { browser } from '$app/env';
	import init from 'wasm-dfsolver';
	import { Shadow } from 'svelte-loading-spinners';

	let is_critical_error: boolean = false;

	if (!$is_wasm_asset_fetched) {
		(async () => {
			if (browser) {
				// Only run code on client side and not server side
				await init().then(
					// Success result
					() => {
						// WASM asset has been downloaded and ready to use

						$is_wasm_asset_fetched = true;
					},
					// Failure result
					() => {
						console.log('A unspecified failure occured.');
						is_critical_error = true;
					}
				);
			}
		})();
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

{#if !is_critical_error && $is_wasm_asset_fetched}
	<div class="content centred">
		<section class="row">
			<div class="column left">
				<Menu />
			</div>
			<div class="column right">
				<Calendar />
			</div>
		</section>
	</div>
{:else if !is_critical_error && !$is_wasm_asset_fetched}
	<div class="loading_spinner centred">
		<Shadow size="60" unit="px" />
	</div>
{:else}
	<h1>WARNING: A critical error has occured. Try refreshing the page.</h1>
{/if}

<style>
	.row {
		display: flex;
	}

	.column {
		float: left;
	}

	.left {
		margin-right: 6rem;
	}

	.right {
		width: min-content;
	}

	.centred {
		/* Center vertically and horizontally */
		display: flex;
		align-items: center;
	}

	.content {
		width: 100%;
		max-width: fit-content;
		padding-top: 2rem;
	}

	.loading_spinner {
		font-family: arial;
		font-size: 24px;
		margin: 25px;
		padding-top: 45%;
		padding-bottom: 10%;
	}

	@media screen and (max-width: 1200px) and (orientation: portrait) {
		.row {
			flex-direction: column;
			flex-flow: column-reverse;
		}

		.left {
			margin-right: 0;
			margin-left: 0rem;
		}

		.column {
			/* Center vertically and horizontally */
			display: flex;
			align-items: center;
		}
	}

	@media screen and (max-height: 760px) and (orientation: landscape) {
		.content {
			padding-top: 0rem;
		}
	}
</style>
