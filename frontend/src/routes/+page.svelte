<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import FileUploader from '$lib/components/FileUploader.svelte';
	import TransformationConfig from '$lib/components/TransformationConfig.svelte';
	import ResultsViewer from '$lib/components/ResultsViewer.svelte';
	import { hasData, hasResult } from '$lib/stores/processing';
</script>

<svelte:head>
	<title>Deterministic Data Processor - Zero Errors by Design</title>
	<meta name="description" content="A permanently usable, multi-user beneficial application that guarantees deterministic behavior and zero runtime errors by design." />
</svelte:head>

<Header />

<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<div class="mb-8">
		<p class="text-gray-600 text-center max-w-3xl mx-auto">
			Upload your data, configure a transformation, and receive deterministic results with guaranteed reproducibility. No databases, no runtime errors, just pure functional processing.
		</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
		<div class="space-y-6">
			<FileUploader />
			{#if $hasData}
				<div class="animate-fade-in">
					<TransformationConfig />
				</div>
			{/if}
		</div>
		
		<div>
			{#if $hasResult}
				<ResultsViewer />
			{:else if $hasData}
				<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-12 text-center">
					<svg class="mx-auto h-16 w-16 text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
					</svg>
					<h3 class="text-lg font-medium text-gray-900 mb-2">Ready to Process</h3>
					<p class="text-sm text-gray-500">
						Select a transformation type and configure the parameters, then click "Process Data" to see your results.
					</p>
				</div>
			{:else}
				<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-12 text-center">
					<svg class="mx-auto h-16 w-16 text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
					</svg>
					<h3 class="text-lg font-medium text-gray-900 mb-2">Get Started</h3>
					<p class="text-sm text-gray-500">
						Upload a CSV or JSON file to begin processing your data with deterministic transformations.
					</p>
				</div>
			{/if}
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-12">
		<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
			<div class="flex items-center space-x-3 mb-3">
				<div class="flex-shrink-0">
					<svg class="h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-gray-900">Zero Runtime Errors</h3>
			</div>
			<p class="text-sm text-gray-600">
				Strong compile-time guarantees and pure functional processing eliminate an entire class of bugs and runtime failures.
			</p>
		</div>

		<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
			<div class="flex items-center space-x-3 mb-3">
				<div class="flex-shrink-0">
					<svg class="h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-gray-900">Fully Deterministic</h3>
			</div>
			<p class="text-sm text-gray-600">
				Identical inputs always produce identical outputs. Perfect for reproducible research, compliance, and testing.
			</p>
		</div>

		<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
			<div class="flex items-center space-x-3 mb-3">
				<div class="flex-shrink-0">
					<svg class="h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-gray-900">No External Dependencies</h3>
			</div>
			<p class="text-sm text-gray-600">
				No database, no network calls during processing. All computation happens in-memory with zero external failure points.
			</p>
		</div>
	</div>
</main>

<footer class="bg-white border-t border-gray-200 mt-12">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
		<p class="text-center text-sm text-gray-500">
			Built with Rust, C#, and SvelteKit for maximum reliability and deterministic behavior.
		</p>
	</div>
</footer>
