<script lang="ts">
	import { onMount } from 'svelte';
	import { fileUploadState, setUploadedData, setUploadError, setUploadLoading } from '../stores/processing';
	import type { DataRow } from '../types';

	let fileInput: HTMLInputElement;
	let Alpine: any;

	onMount(async () => {
		const alpineModule = await import('alpinejs');
		Alpine = alpineModule.default;
		
		if (!window.Alpine) {
			window.Alpine = Alpine;
			Alpine.start();
		}
	});

	function handleFileSelect(event: Event) {
		const target = event.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			processFile(target.files[0]);
		}
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
			processFile(event.dataTransfer.files[0]);
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
	}

	async function processFile(file: File) {
		setUploadLoading(true);
		
		try {
			const extension = file.name.split('.').pop()?.toLowerCase();
			
			if (extension !== 'csv' && extension !== 'json') {
				throw new Error('Only CSV and JSON files are supported');
			}
			
			const text = await file.text();
			let data: DataRow[];
			
			if (extension === 'csv') {
				data = parseCSV(text);
			} else {
				data = parseJSON(text);
			}
			
			if (data.length === 0) {
				throw new Error('File contains no data');
			}
			
			setUploadedData(data);
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Failed to process file';
			setUploadError(errorMessage);
		} finally {
			setUploadLoading(false);
		}
	}

	function parseCSV(text: string): DataRow[] {
		const lines = text.trim().split('\n');
		if (lines.length < 2) {
			throw new Error('CSV file must contain headers and at least one data row');
		}
		
		const headers = lines[0].split(',').map(h => h.trim().replace(/^"|"$/g, ''));
		const data: DataRow[] = [];
		
		for (let i = 1; i < lines.length; i++) {
			const line = lines[i].trim();
			if (!line) continue;
			
			const values = line.split(',').map(v => v.trim().replace(/^"|"$/g, ''));
			const row: DataRow = {};
			
			headers.forEach((header, index) => {
				const value = values[index] || '';
				const numValue = parseFloat(value);
				row[header] = isNaN(numValue) ? value : numValue;
			});
			
			data.push(row);
		}
		
		return data;
	}

	function parseJSON(text: string): DataRow[] {
		const parsed = JSON.parse(text);
		
		if (!Array.isArray(parsed)) {
			throw new Error('JSON file must contain an array of objects');
		}
		
		if (parsed.length === 0 || typeof parsed[0] !== 'object') {
			throw new Error('JSON file must contain an array of objects');
		}
		
		return parsed as DataRow[];
	}
</script>

<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<h2 class="text-lg font-semibold text-gray-900 mb-4">Upload Data</h2>
	
	<div
		class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-primary-400 transition-colors duration-200 cursor-pointer"
		on:drop={handleDrop}
		on:dragover={handleDragOver}
		role="button"
		tabindex="0"
		on:click={() => fileInput.click()}
		on:keydown={(e) => e.key === 'Enter' && fileInput.click()}
		x-data="{ 'dragging': false }"
		x-on:dragenter="dragging = true"
		x-on:dragleave="dragging = false"
		x-on:drop="dragging = false"
		x-bind:class="dragging ? 'border-primary-500 bg-primary-50' : ''"
	>
		{#if $fileUploadState.isLoading}
			<div class="flex flex-col items-center space-y-3">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500"></div>
				<p class="text-sm text-gray-600">Processing file...</p>
			</div>
		{:else if $fileUploadState.data.length > 0}
			<div class="flex flex-col items-center space-y-3">
				<svg class="w-12 h-12 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<div>
					<p class="text-sm font-medium text-gray-900">File uploaded successfully</p>
					<p class="text-sm text-gray-500">{$fileUploadState.data.length} rows loaded</p>
				</div>
				<button
					type="button"
					class="mt-2 px-4 py-2 text-sm font-medium text-primary-700 bg-primary-50 rounded-lg hover:bg-primary-100 transition-colors duration-200"
					on:click|stopPropagation={() => fileInput.click()}
				>
					Upload Different File
				</button>
			</div>
		{:else}
			<div class="flex flex-col items-center space-y-3">
				<svg class="w-12 h-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
				</svg>
				<div>
					<p class="text-sm font-medium text-gray-900">Drop your file here or click to browse</p>
					<p class="text-xs text-gray-500 mt-1">Supports CSV and JSON files (max 10MB)</p>
				</div>
			</div>
		{/if}
	</div>
	
	{#if $fileUploadState.error}
		<div class="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg">
			<div class="flex items-start space-x-3">
				<svg class="w-5 h-5 text-red-500 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<div class="flex-1">
					<h3 class="text-sm font-medium text-red-800">Upload Error</h3>
					<p class="text-sm text-red-700 mt-1">{$fileUploadState.error}</p>
				</div>
			</div>
		</div>
	{/if}
	
	<input
		type="file"
		bind:this={fileInput}
		on:change={handleFileSelect}
		accept=".csv,.json"
		class="hidden"
	/>
</div>
