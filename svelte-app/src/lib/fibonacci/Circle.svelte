<script lang="ts">
	import { getContext, onMount } from 'svelte';

	export let r: number;
	export let cx: number;
	export let cy: number;
	export let fill: string;

	const { register, unregister } = getContext('canvas');

	onMount(() => {
		register(draw);

		return () => {
			unregister(draw);
		};
	});

	function draw(ctx: CanvasRenderingContext2D) {
		ctx.beginPath();
		ctx.fillStyle = fill;
		ctx.arc(cx, cy, r, 0, 2 * Math.PI, true);
		ctx.fill();
	}
</script>
