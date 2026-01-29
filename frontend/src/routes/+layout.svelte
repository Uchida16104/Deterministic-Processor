<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';

	onMount(async () => {
		const htmx = await import('htmx.org');
		const Alpine = (await import('alpinejs')).default;
		
		if (typeof window !== 'undefined') {
			(window as any).htmx = htmx;
			if (!(window as any).Alpine) {
				(window as any).Alpine = Alpine;
				Alpine.start();
			}
		}
	});
</script>

<div class="min-h-screen bg-gray-50">
	<slot />
</div>
