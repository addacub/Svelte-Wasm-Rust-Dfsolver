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
		padding: 5rem 1rem 1rem 2rem;
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
		padding: 10rem 0;
	}

	footer a {
		font-weight: bold;
	}

	@media (max-width: 1200px) {
		footer {
			padding-top: 2rem;
			font-size: small;
		}
	}

	@media (max-width: 620px) {
		footer {
			padding-top: 4rem;
			font-size: small;
		}
	}

	@media screen and (max-width: 620px) and (orientation: portrait) {
		main {
			padding: 5rem 0rem 1rem 0rem;
		}
	}
</style>
