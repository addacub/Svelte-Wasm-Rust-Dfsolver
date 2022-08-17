<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import '../app.css';
</script>

<Header />

<main>
	<slot />
</main>

<footer>
	<p>visit <a href="https://github.com/addacub">GitHub</a> to explore other projects</p>
</footer>

<style>
	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 2rem 1rem 1rem 1rem;
		width: 100%;
		max-width: fit-content;
		margin: 0 auto;
		box-sizing: border-box;
		background-color: 19232D;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 5rem 0;
	}

	footer a {
		font-weight: bold;
	}

	@media (max-width: 1200px) {
		main {
			padding: 2rem 3rem 1rem 3rem;
		}

		footer {
			padding: 2rem 0;
			font-size: small;
		}
	}

	@media (max-width: 620px) {
		main {
			padding: 1rem 1.5rem 1rem 1.5rem;
		}
	}
</style>
